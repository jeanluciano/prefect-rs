//! This module contains a dedicated thread pool for running "cpu
//! intensive" workloads such as DataFusion plans
#![deny(rustdoc::broken_intra_doc_links, rustdoc::bare_urls, rust_2018_idioms)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::explicit_iter_loop,
    clippy::future_not_send,
    clippy::use_self,
    clippy::clone_on_ref_ptr
)]

use parking_lot::Mutex;
use pin_project::{pin_project, pinned_drop};
use std::{pin::Pin, sync::Arc};
use tokio::sync::oneshot::{error::RecvError, Receiver};
use tokio_util::sync::CancellationToken;
use futures::{
    future::{BoxFuture, Shared},
    Future, FutureExt, TryFutureExt,
};
use tracing::warn;


/// Task that can be added to the executor-internal queue.
///
/// Every task within the executor is represented by a [`Job`] that can be polled by the API user.
struct Task {
    fut: Pin<Box<dyn Future<Output = ()> + Send>>,
    cancel: CancellationToken,

    #[allow(dead_code)]
    task_ref: Arc<()>,
}

impl Task {
    /// Run task.
    ///
    /// This runs the payload or cancels if the linked [`Job`] is dropped.
    async fn run(self) {
        tokio::select! {
            _ = self.cancel.cancelled() => (),
            _ = self.fut => (),
        }
    }
}

/// The type of error that is returned from tasks in this module
pub type Error = tokio::sync::oneshot::error::RecvError;

/// Job within the executor.
///
/// Dropping the job will cancel its linked task.
#[pin_project(PinnedDrop)]
#[derive(Debug)]
pub struct Job<T> {
    cancel: CancellationToken,
    detached: bool,
    #[pin]
    rx: Receiver<T>,
}

impl<T> Job<T> {
    /// Detached job so dropping it does not cancel it.
    ///
    /// You must ensure that this task eventually finishes, otherwise [`DedicatedExecutor::join`] may never return!
    pub fn detach(mut self) {
        // cannot destructure `Self` because we implement `Drop`, so we use a flag instead to prevent cancelation.
        self.detached = true;
    }
}

impl<T> Future for Job<T> {
    type Output = Result<T, Error>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        this.rx.poll(cx)
    }
}

#[pinned_drop]
impl<T> PinnedDrop for Job<T> {
    fn drop(self: Pin<&mut Self>) {
        if !self.detached {
            self.cancel.cancel();
        }
    }
}

/// Runs futures (and any `tasks` that are `tokio::task::spawned` by
/// them) on a separate tokio Executor
#[derive(Clone)]
pub struct DedicatedExecutor {
    state: Arc<Mutex<State>>,
}

/// Runs futures (and any `tasks` that are `tokio::task::spawned` by
/// them) on a separate tokio Executor
struct State {
    /// Channel for requests -- the dedicated executor takes requests
    /// from here and runs them.
    ///
    /// This is `None` if we triggered shutdown.
    requests: Option<std::sync::mpsc::Sender<Task>>,

    /// Receiver side indicating that shutdown is complete.
    completed_shutdown: Shared<BoxFuture<'static, Result<(), Arc<RecvError>>>>,

    /// Task counter (uses Arc strong count).
    task_refs: Arc<()>,

    /// The inner thread that can be used to join during drop.
    thread: Option<std::thread::JoinHandle<()>>,
}

// IMPORTANT: Implement `Drop` for `State`, NOT for `DedicatedExecutor`, because the executor can be cloned and clones
// share their inner state.
impl Drop for State {
    fn drop(&mut self) {
        if self.requests.is_some() {
            warn!("DedicatedExecutor dropped without calling shutdown()");
            self.requests = None;
        }

        // do NOT poll the shared future if we are panicking due to https://github.com/rust-lang/futures-rs/issues/2575
        if !std::thread::panicking() && self.completed_shutdown.clone().now_or_never().is_none() {
            warn!("DedicatedExecutor dropped without waiting for worker termination",);
        }

        // join thread but don't care about the results
        self.thread.take().expect("not dropped yet").join().ok();
    }
}

/// The default worker priority (value passed to `libc::setpriority`);
const WORKER_PRIORITY: i32 = 10;

impl std::fmt::Debug for DedicatedExecutor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Avoid taking the mutex in debug formatting
        write!(f, "DedicatedExecutor")
    }
}

impl DedicatedExecutor {
    /// Creates a new `DedicatedExecutor` with a dedicated tokio
    /// executor that is separate from the threadpool created via
    /// `[tokio::main]` or similar.
    ///
    /// The worker thread priority is set to low so that such tasks do
    /// not starve other more important tasks (such as answering health checks)
    ///
    /// Follows the example from to stack overflow and spawns a new
    /// thread to install a Tokio runtime "context"
    /// <https://stackoverflow.com/questions/62536566>
    ///
    /// If you try to do this from a async context you see something like
    /// thread 'plan::stringset::tests::test_builder_plan' panicked at 'Cannot
    /// drop a runtime in a context where blocking is not allowed. This
    /// happens when a runtime is dropped from within an asynchronous
    /// context.', .../tokio-1.4.0/src/runtime/blocking/shutdown.rs:51:21
    pub fn new(thread_name: &str, num_threads: usize) -> Self {
        let thread_name = thread_name.to_string();

        let (tx_tasks, rx_tasks) = std::sync::mpsc::channel::<Task>();
        let (tx_shutdown, rx_shutdown) = tokio::sync::oneshot::channel();

        let thread = std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .thread_name(&thread_name)
                .worker_threads(num_threads)
                .on_thread_start(move || set_current_thread_priority(WORKER_PRIORITY))
                .build()
                .expect("Creating tokio runtime");

            runtime.block_on(async move {
                // Dropping the tokio runtime only waits for tasks to yield not to complete
                //
                // We therefore use a RwLock to wait for tasks to complete
                let join = Arc::new(tokio::sync::RwLock::new(()));

                while let Ok(task) = rx_tasks.recv() {
                    let join = Arc::clone(&join);
                    let handle = join.read_owned().await;

                    tokio::task::spawn(async move {
                        task.run().await;
                        std::mem::drop(handle);
                    });
                }

                // Wait for all tasks to finish
                join.write().await;

                // signal shutdown, but it's OK if the other side is gone
                tx_shutdown.send(()).ok();
            })
        });

        let state = State {
            requests: Some(tx_tasks),
            task_refs: Arc::new(()),
            completed_shutdown: rx_shutdown.map_err(Arc::new).boxed().shared(),
            thread: Some(thread),
        };

        Self {
            state: Arc::new(Mutex::new(state)),
        }
    }

    /// Runs the specified Future (and any tasks it spawns) on the
    /// `DedicatedExecutor`.
    ///
    /// Currently all tasks are added to the tokio executor
    /// immediately and compete for the threadpool's resources.
    pub fn spawn<T>(&self, task: T) -> Job<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
    {
        let (tx, rx) = tokio::sync::oneshot::channel();

        let fut = Box::pin(async move {
            let task_output = task.await;
            if tx.send(task_output).is_err() {
                warn!("Spawned task output ignored: receiver dropped")
            }
        });
        let cancel = CancellationToken::new();
        let mut state = self.state.lock();
        let task = Task {
            fut,
            cancel: cancel.clone(),
            task_ref: Arc::clone(&state.task_refs),
        };

        if let Some(requests) = &mut state.requests {
            // would fail if someone has started shutdown
            requests.send(task).ok();
        } else {
            warn!("tried to schedule task on an executor that was shutdown");
        }

        Job {
            rx,
            cancel,
            detached: false,
        }
    }

    /// Number of currently active tasks.
    pub fn tasks(&self) -> usize {
        let state = self.state.lock();

        // the strong count is always `1 + jobs` because of the Arc we hold within Self
        Arc::strong_count(&state.task_refs).saturating_sub(1)
    }

    /// signals shutdown of this executor and any Clones
    pub fn shutdown(&self) {
        // hang up the channel which will cause the dedicated thread
        // to quit
        let mut state = self.state.lock();
        state.requests = None;
    }

    /// Stops all subsequent task executions, and waits for the worker
    /// thread to complete. Note this will shutdown all clones of this
    /// `DedicatedExecutor` as well.
    ///
    /// Only the first all to `join` will actually wait for the
    /// executing thread to complete. All other calls to join will
    /// complete immediately.
    ///
    /// # Panic / Drop
    /// [`DedicatedExecutor`] implements shutdown on [`Drop`]. You should just use this behavior and NOT call
    /// [`join`](Self::join) manually during [`Drop`] or panics because this might lead to another panic, see
    /// <https://github.com/rust-lang/futures-rs/issues/2575>.
    pub async fn join(&self) {
        self.shutdown();

        // get handle mutex is held
        let handle = {
            let state = self.state.lock();
            state.completed_shutdown.clone()
        };

        // wait for completion while not holding the mutex to avoid
        // deadlocks
        handle.await.expect("Thread died?")
    }
}

#[cfg(unix)]
fn set_current_thread_priority(prio: i32) {
    // on linux setpriority sets the current thread's priority
    // (as opposed to the current process).
    unsafe { libc::setpriority(0, 0, prio) };
}

#[cfg(not(unix))]
fn set_current_thread_priority(prio: i32) {
    warn!("Setting worker thread priority not supported on this platform");
}
// struct PrefectFuture {
//     task_run: TaskRun,
//     task_run_id: UUID,
//     task_runner: TaskRunner,
//     asynchronous: bool,
//     _final_state: State,
//     _exception: Option<Exception>,
// }

// trait Future {
//     fn wait(self, timeout: f32) -> Awaitable;
//     fn result(self, timeout: f32) -> Awaitable;
//     fn get_state(self) -> State;
// }

// fn resolve_futures_to_data(expr: PrefectFuture) -> TokenStream {}

// fn resolve_futures_to_states(expr: PrefectFuture) -> TokenStream {}

// fn call_repr(_fn: ItemFn, kwargs: HashMap) -> str {}

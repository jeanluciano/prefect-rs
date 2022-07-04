use crate::futures::PrefectFuture;
use crate::types::core::TaskRun;
use async_trait::async_trait;
use std::{
    any::Any,
    collections::{HashMap, HashSet},
};
use uuid::Uuid;

pub struct ConcurrentTaskRunner {
    // logger: Logger,
    started: bool,
    _task_group: TaskGroup,
    _results: HashMap<Uuid, dyn Any>,
    _task_run_ids: HashSet<Uuid>,
}

impl ConcurrentTaskRunner {
    async fn _run_and_store_result(
        self,
        task_run_id: Uuid,
        run_fn: FnMut(),
        run_kwargs: HashMap<String, String>,
    ) {
        self._results[task_run_id] = run_fn(run_kwargs).await?;
    }
    async fn _get_run_result(self, task_run_id: Uuid, timeout: f32) -> State {
        run_fn(run_kwargs).await?
    }
}

#[async_trait]
impl Runner for ConcurrentTaskRunner {
    async fn start(self) {
        if self._started {
            panic!("The task runner is already started!")
        }
    }
    async fn submit(
        self,
        task_run: TaskRun,
        task_fn: FnMut,
        run_kwargs: HashMap<String, String>,
        asynchronous: bool,
    ) -> PrefectFuture {
        if !self._started {
            panic!("The task runner must be started before submitting work.")
        }
        if !self._task_group {
            panic!()
        }

        self._task_run_ids.add(task_run.id);

        PrefectFuture::new(task_run, self, asynchronous)
    }
    async fn wait(self, prefect_future: PrefectFuture, timeout: f32) -> Option<State> {
        if !self._task_group {
            panic!(
                "The concurrent task runner cannot be used to wait for work after serialization."
            )
        }
        return self._get_run_result(prefect_future.task_run.id, timeout);
    }
}

trait Runner {
    fn concurrency_type(self) -> TaskConcurrecyType;
    fn name(self) -> str;
    async fn submit(self, TaskSubmit: TaskSubmit) -> PrefectFuture;
    async fn wait(self, prefect_future: PrefectFuture, timeout: f32) -> State;
    async fn start(self) -> AsyncIterator;
}

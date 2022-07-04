use crate::client::OrionClient;
use crate::flows::Flow;
use chrono::{DateTime, Utc};
use crate::types::core::{FlowRun, TaskRun};
use crate::task_runners::ConcurrentTaskRunner;
use crate::tasks::Task;
enum ContextType {
    FlowRun,
    TaskRun
}

#[derive(Debug)]
struct RunContext {
    start_time: DateTime<Utc>,
    client: OrionClient
}



#[derive(Debug)]
pub struct FlowRunContext<F> 
{
    flow: Flow<F>,
    flow_run: FlowRun<F>,
    task_runner: ConcurrentTaskRunner,
    // result_storage:StorageBlock,
    // task_run_futures: Vec<PrefectFuture>,
    // subflow_states: Vec<State>,
    // sync_portal: Option<BlockingPotal>,
    // timeout_scope: Option<CancelScope>,
    // __var: ContextType,
    start_time: DateTime<Utc>,
    client: OrionClient
} 

#[derive(Debug)]
pub struct TaskRunContext<F> {
    task: Task<F>,
    task_run: TaskRun,
    // result_storage:StorageBlock,
    // __var: ContextType,
    start_time: DateTime<Utc>,
    client: OrionClient
}



// fn get_run_context() -> Union<FlowRunContext,TaskRunContext>;
// fn get_settings_context() -> SettingsContext;

// fn use_profile(profile: Union<Profile,String>, override_enviroment_variables: bool,include_current_context: bool);
// fn enter_root_settings_context();


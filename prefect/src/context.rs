
use tokio::sync::{RwLock};
use std::sync::{Arc};
// use crate::client::OrionClient;
use crate::flows::Flow;
use chrono::{DateTime, Utc};
use crate::types::core::{FlowRun};
// use crate::task_runners::ConcurrentTaskRunner;
// use crate::tasks::Task;


enum ContextType {
    FlowRun,
    TaskRun
}
#[derive(Debug)]
pub struct FlowRunContext
{
    flow: Flow,
    flow_run: FlowRun,
    // task_runner: ConcurrentTaskRunner,
    // result_storage:StorageBlock,
    // task_run_futures: Vec<PrefectFuture>,
    // subflow_states: Vec<State>,
    // sync_portal: Option<BlockingPotal>,
    // timeout_scope: Option<CancelScope>,
    // __var: ContextType,
    start_time: DateTime<Utc>,
    // client: OrionClient
} 

impl FlowRunContext {
    fn new(flow: Flow,flow_run: FlowRun,start_time:DateTime<Utc>) -> Arc<RwLock<FlowRunContext>> {
        Arc::new(RwLock::new(FlowRunContext{
            flow,
            flow_run,
            start_time
        }))
    }
}

// #[derive(Debug)]
// pub struct TaskRunContext {
//     task: Task,
//     task_run: TaskRun,
//     // result_storage:StorageBlock,
//     // __var: ContextType,
//     start_time: DateTime<Utc>,
//     client: OrionClient
// }



// fn get_run_context() -> Union<FlowRunContext,TaskRunContext>;
// fn get_settings_context() -> SettingsContext;

// fn use_profile(profile: Union<Profile,String>, override_enviroment_variables: bool,include_current_context: bool);
// fn enter_root_settings_context();


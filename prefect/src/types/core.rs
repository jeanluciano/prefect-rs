
use std::collections::HashMap;

use uuid::Uuid;
use chrono::{DateTime,Utc};

use crate::{context::FlowRunContext, flows::Parameters};

use super::states::{StateType, State};

struct IDBaseModel {
    id: Uuid
}
struct ORMBaseModel {
    created: DateTime<Utc>,
    updated: DateTime<Utc>

}
struct Flow {
    name: String,
    tags:Vec<String>
}

struct FlowRunnerSettings {
    flow_runner_type:String,
    config: HashMap<String,String>
}

pub struct FlowRun<F> {
    name: String,
    flow_id: Uuid,
    state_id: Uuid,
    deployment_id: Uuid,
    flow_version: String,
    parameters: Parameters,
    idempotency_key: String,
    context: FlowRunContext<F>,
    empirical_policy:HashMap<String,String>,
    empirical_configt: HashMap<String,String>,
    tags: Vec<String>,
    parent_task_run_id: Uuid,
    state_type: StateType,
    run_countt: i32,
    expected_start_time: DateTime<Utc>,
    next_scheduled_start_time: DateTime<Utc>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    total_run_time: DateTime<Utc>,
    estimated_run_time:DateTime<Utc>,
    estimated_start_time: DateTime<Utc>,
    auto_scheduled: bool,
    flow_runner: FlowRunnerSettings,
    state: State
}

struct TaskRunPolicy { 
    max_retries: i32,
    retry_delay_seconds: f32
}

struct TaskRunInput {
    input_type: String
}

struct TaskRunResult {
    id: Uuid,
    input_type: InputType,
    name: String
}
struct Parameter {
    input_type: InputType,
    name: String,
}

struct Constant { 
    input_type: InputType
}

pub struct TaskRun {
    name: String,
    flow_run_id: Uuid,
    task_key: String,
    dynamic_key: String,
    cache_key: String,
    cache_expiration: DateTime<Utc>,
    task_version: String,
    empirical_policy: TaskRunPolicy,
    tags: Vec<String>,
    state_id: Uuid,
    task_inputs: TaskInputs,
    state_type: StateType,
    run_count: i32,
    expected_start_time: DateTime<Utc>,
    next_scheduled_start_time: DateTime<Utc>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    total_run_time: DateTime<Utc>,
    estimated_run_time: DateTime<Utc>,
    estimated_start_time: DateTime<Utc>,
    state: State,
}

struct Deployment {
    name: String,
    flow_id: Uuid,
    schedule: SCHEDULE_TYPES,
    is_schedule_active: bool,
    parameters: Parameters,
    tags: [String],
    flow_runner: FlowRunnerSettings
}

// struct ConcurrencyLimit { 
//     tag: String,
//     concurrency_limit: i32,
//     active_slots: [Uuid],
// }

// struct BlockSpect {
//     name: String,
//     version: String,
//     block_type: String,
//     fields: HashMap,
// }

// struct Block {
//     name: String,
//     data: HashMap,
//     block_spec_id: Uuid,
//     block_spec: [BlockSpect]
// }

// struct Configuration {
//     key: String,
//     value: HashMap
// }

// struct SavedSearchFilter {
//     object: String,
//     property: String,
//     filter_type: String,
//     operation: String,
//     value: Any,
// }

// struct SavedSearch {
//     name: String,
//     filters: [SavedSearchFilter]
// }

// struct Log {
//     name: String,
//     level: i32,
//     message: String,
//     timestamp: DateTime<Utc>,
//     flow_run_id: Uuid,
//     task_run_id: UuidOpi
// }

// struct QueueFilter {
//     tags: [String],
//     deployment_ids: [Uuid],
//     flow_runner_types: [String]
// }

// struct WorkQueue {
//     queue_filter: QueueFilter,
//     name: String,
//     description: [String],
//     is_paused: bool,
//     concurrency_limit: [int]
// }

// struct Agent {
//     name: String,
//     work_queue_id: Uuid,
//     last_activity_time: DateTime<Utc>
// }
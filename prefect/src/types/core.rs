use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use uuid::Uuid;

use crate::{context::FlowRunContext, flows::Parameters};

use crate::types::states::{State, StateType};

#[derive(Deserialize)]
pub struct Flow {
    pub id: Uuid,
    pub version: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub name: String,
    pub tags: Vec<String>,
}

struct FlowRunnerSettings {
    flow_runner_type: String,
    config: HashMap<String, String>,
}
#[derive(Debug,Serialize,Deserialize)]
pub struct FlowRun {
    pub name: String,
    pub flow_id: Uuid,
    pub state_id: Option<Uuid>,
    pub deployment_id: Option<Uuid>,
    pub flow_version: String,
    pub parameters: HashMap<String, String>,
    pub idempotency_key: Option<String>,
    pub context: HashMap<String, String>,
    pub empirical_policy: Option<HashMap<String, String>>,
    pub empirical_config: Option<HashMap<String, String>>,
    pub tags: Vec<String>,
    pub parent_task_run_id: Uuid,
    pub state_type: Option<StateType>,
    pub run_count: i32,
    pub expected_start_time: Option<DateTime<Utc>>,
    pub next_scheduled_start_time: Option<DateTime<Utc>>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub total_run_time: Option<DateTime<Utc>>,
    pub estimated_run_time: Option<DateTime<Utc>>,
    pub estimated_start_time: Option<DateTime<Utc>>,
    pub auto_scheduled: bool,
    // flow_runner: FlowRunnerSettings,
    pub state: State,
}

impl FlowRun {
    pub fn new(
        
        name: String,
        flow_id: Uuid,
        flow_version: String,
        parameters: HashMap<String, String>,
        tags: Vec<String>,
        state: State,
        // flow_runner,
        context: HashMap<String, String>,
        parent_task_run_id: Uuid,
    ) -> FlowRun {
        let state_id = None;
        let deployment_id = None;
        let idempotency_key = None;
        let empirical_policy = None;
        let empirical_config = None;
        let state_type = None;
        let expected_start_time = None;
        let next_scheduled_start_time = None;
        let start_time = None;
        let end_time = None;
        let total_run_time = None;
        let estimated_run_time = None;
        let estimated_start_time = None;
        let auto_scheduled = false;
        let auto_scheduled = false;
        let run_count = 0;
        Self {
            name,

            flow_id,
            state_id,
            deployment_id,
            flow_version,
            parameters,
            idempotency_key,
            context,
            empirical_policy,
            empirical_config,
            tags,
            parent_task_run_id,
            state_type,
            run_count,
            expected_start_time,
            next_scheduled_start_time,
            start_time,
            end_time,
            total_run_time,
            estimated_run_time,
            estimated_start_time,
            auto_scheduled,
            state,
        }
    }
}

struct TaskRunPolicy {
    max_retries: i32,
    retry_delay_seconds: f32,
}

struct TaskRunInput {
    input_type: String,
}

// struct TaskRunResult {
//     id: Uuid,
//     input_type: InputType,
//     name: String
// }
// struct Parameter {
//     input_type: InputType,
//     name: String,
// }

// struct Constant {
//     input_type: InputType
// }

// pub struct TaskRun {
//     name: String,
//     flow_run_id: Uuid,
//     task_key: String,
//     dynamic_key: String,
//     cache_key: String,
//     cache_expiration: DateTime<Utc>,
//     task_version: String,
//     empirical_policy: TaskRunPolicy,
//     tags: Vec<String>,
//     state_id: Uuid,
//     task_inputs: TaskInputs,
//     state_type: StateType,
//     run_count: i32,
//     expected_start_time: DateTime<Utc>,
//     next_scheduled_start_time: DateTime<Utc>,
//     start_time: DateTime<Utc>,
//     end_time: DateTime<Utc>,
//     total_run_time: DateTime<Utc>,
//     estimated_run_time: DateTime<Utc>,
//     estimated_start_time: DateTime<Utc>,
//     state: State,
// }

// struct Deployment {
//     name: String,
//     flow_id: Uuid,
//     schedule: SCHEDULE_TYPES,
//     is_schedule_active: bool,
//     parameters: Parameters,
//     tags: [String],
//     flow_runner: FlowRunnerSettings
// }

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

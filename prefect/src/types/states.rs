use chrono::{DateTime, Utc};
use uuid::Uuid;

pub enum StateType {
    SCHEDULED,
    PENDING,
    RUNNING,
    COMPLETED,
    FAILED,
    CANCELLED
}

enum TERMINAL_STATES  {
    COMPLETED,
    FAILED,
    CANCELLED
}

struct StateDetails {
    flow_run_id: Uuid,
    task_run_id: Uuid,
    child_flow_run_id: Uuid,
    schedule_time: DateTime<Utc>,
    cache_key: String,
    cache_expiration: DateTime<Utc>
}

pub struct State {
    id: uuid::Uuid,
    state_type: StateType,
    name: String,
    timestamp:DateTime<Utc>,
    message: String,
    // data: DataDocument,
    state_details: StateDetails
}
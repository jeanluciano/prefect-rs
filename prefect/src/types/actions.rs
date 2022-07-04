
struct FlowCreate {
    name:String,
    tags: Vec<String>
}

struct FlowUpdate {
    name:String,
    tags: Vec<String>
}

struct DeploymentCreate {
    name: String,
    flow_id: Byte,
    schedule: SCHEDULE_TYPES,
    is_schedule_active: bool,
    parameters: Parameters,
    tags: Vec<String>,
    flow_runner: FlowRunnerSettings,
    flow_data: DataDocument
}

struct FlowFunUpdate {
    name: STATIC,
    flow_version: String,
    parameters: HashMap,
    flow_runner: String
}

struct StateCreate {
    
}

use uuid::Uuid;
use chrono::{DateTime,Utc};

trait PrefectFilterBaseModel {
    fn as_sql_filter(&self, db:OrionDbInterface) -> BooleanClauseList {

    }
    fn _get_filter_list(&self, db:OrionDbInterface) -> Vector{

    }
}

struct FlowFilterId {
    any_: Vec<Uuid>,
}

struct FlowFilterName {
    any_: Vec<String>
}

struct FlownFilterTags {
    all_: bool,
    is_null_: bool,
}

struct FlowFilter {
    id: FlowFilterId,
    name: FlowFilterName,
    tags: FlownFilterTags
}

struct FlowRunFilterId {
    any_: Vec<Uuid>,
    not_any_: Vec<Uuid>,
}

struct FlowRunFilterName {
    any_: Vec<String>
}

struct FlowRunFilterTags {
    all_:Vec<String>,
    is_null_: bool
}

struct FlowRunFilterDeploymentId {
    any_: Vec<Uuid>,
    is_null_: bool
}

struct FlowRunFilterStateType {
    any_: [List]
}

struct FlowRunFilterStateName {
    any_: Vec<String>
}

struct FlowRunFilterState {
    filter_type: Option<FlowRunFilterStateType>,
    name: Option<FlowRunFilterName>
}

struct FlowRunFilterFlowVersion {
    any_: Vec<String>
}

struct FlowRunFilterStartTime {
    before_: DateTime<Utc>,
    after_: DateTime<Utc>,
    is_null_: bool
}

struct FlowRunFilterExpectedStartTime {
    before_: DateTime<Utc>,
    after_: DateTime<Utc>,
}

struct FlowRunFilterNextScheduledStartTime {
    before_: DateTime<Utc>,
    after_: DateTime<Utc>,
}

struct FlowRunFilterParentTaskRunId {
    any_: Vec<Uuid>,
    is_null_: bool
}

struct FlowRunFilterFlowRunnerType {
    any_: Vec<String>
}

struct FlowRunFilter {
    id: Option<FlowRunFilterId>,
    name: Option<FlowRunFilterName>,
    tags: Option<FlowRunFilterTags>,
    deployment_id: Option<FlowRunFilterDeploymentId>,
    state: Option<FlowRunFilterState>,
    flow_version: Option<FlowRunFilterFlowVersion>,
    start_time: Option<FlowRunFilterStartTime>,
    expected_start_time: Option<FlowRunFilterExpectedStartTime>,
    next_scheduled_start_time: Option<FlowRunFilterNextScheduledStartTime>,
    parent_task_run_id: Option<FlowRunFilterParentTaskRunId>,
    flow_runner_type: Option<FlowRunFilterFlowRunnerType>
}


struct TaskRunFilterId {
    any_:Vec<Uuid>
}
struct TaskRunFilterName {
    any_: Vec<String>
}

struct TaskRunFilterTags {
    all_: Vec<String>,
    is_null_: bool,
}

struct TaskRunFilterStateType {
    any_: Vec<String>
}

struct TaskRunFilterState {
    any_: Vec<String>
}

struct TaskRunFilterSubFlowRuns {
    exist_:bool
}

struct TaskRunFilterStartTime {
    before_:DateTime<Utc>,
    after_: DateTime<Utc>,
    is_null_: bool
}

struct TaskRunFilter {
    id: Option<TaskRunFilterId>,
    name: Option<TaskRunFilterName>,
    tags: Option<TaskRunFilterTags>,
    state: Option<TaskRunFilterStat>,
    start_time: Option<TaskRunFilterStartTime>,
    subflow_runs: Option<TaskRunFilterSubFlowRuns>
}
struct DeploymentFilterId {
    any_: Vec<Uuid>
}

struct DeploymentFilterName {
    any_: Vec<String>
}

struct DeploymentFilterIsScheduleActive {
    eq_: bool
}

struct DeploymentFilterTags {
    all_: Vec<String>,
    is_null_: bool
}

struct DeploymentFilter {
    id: Option<DeploymentFilterId>,
    name: Option<DeploymentFilterName>,
    is_schedule_active: Option<DeploymentFilterIsScheduleActive>,
    tags: Option<DeploymentFilterTags>
}

struct LogFilterName { 
    any_: Vec<String>
}

struct LogFilterLevel {
    ge_: i32,
    le_: i32,
}

struct LogFilterTimestamp {
    before_: DateTime<Utc>,
    after_: DateTime<Utc>,
}

struct LogFilterFlowRunId {
    any_: Vec<Uuid>
}

struct LogFilterTaskRunId {
    any_: Vec<Uuid>
}

struct LogFilter {
    level: Option<LogFilterLevel>,
    timestamp: Option<LogFilterTimestamp>,
    flow_run_id: Option<LogFilterFlowRunId>,
    task_run_id: Option<LogFilterTaskRunId>,

}

struct FilterSet {
    flows: FlowFilter,
    flow_runs: FlowRunFilter,
    task_runs: TaskRunFilter,
    deployments: DeploymentFilter

}
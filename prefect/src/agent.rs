// struct OrionAgent {
//     work_queue_id: Bytes,
//     work_queue_name: String,
//     prefetch_seconds: i32,
//     submittiing_flow_run_ids:Set,
//     started: bool,
//     logger: Logger,
//     task_group: Optional<TaskGroup>,
//     client: Optional<OrionAgent>
// }

// trait Inititate {
//     fn new(&self,work_queue_id,work_queue_name,prefetch_seconds) -> OrionAgent;
//     fn work_queue_id_from_name(self) -> Optional<Bytes>;
//     fn get_and_submit_flow_runs(self) -> List<FlowRun>;
//     fn get_flow_runner(self, flow_run: FlowRun);
//     fn submit_run(self, flow_run: FlowRun) -> None;
//     fn _propose_pending_state(self, flow_run: FlowRun) -> bool;
//     fn _propose_failed_state(self, flow_run: FlowRun, exc: Exception) -> None;
// }

// trait ManageContext {
//     fn start(&self);
//     fn shutdow(self, exc_info);
    
//     // add code here
// }



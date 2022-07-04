struct FlowRunner{
    typename: str
}

// trait FlowRunnerMethods {
//  fn to_settings(self) -> FlownRunnerSettings;
//  fn from_settings(self, settings: FlownRunnerSettings) -> FlowRunner;
//  fn logger(self) -> Logger;
//  fn submit_flow_run(self, flow_run: FlowRun,task_status:TaskStatus) -> Option<bool>;
//  fn register_flow_runner(flow_runner: FlowRunner) -> FlowRunner;
//  fn look_flow_runner(typename:str) -> FlowRunner;

// }

struct SubprocessFlowRunner {
    typename: str,
    stream_output: bool
}
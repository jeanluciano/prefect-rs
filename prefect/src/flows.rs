use crate::{engine::enter_flow_run_engine_from_flow_call, task_runners::ConcurrentTaskRunner};


pub struct Parameters {
    test: String
}

pub struct Flow<F>
{
    __fn: F,
    name: String,
    version: String,
    task_runner: ConcurrentTaskRunner,
    description: String,
    timeout_seconds: Option<f32>,
    validate_parameters: bool,
}

// impl<F,P,R> Flow<F,P,R> {
//     fn validate_parameters(self, param: Parameters) -> Parameters;
//     fn serialize_parameters(self, param: Parameters) -> Parameters;
// }




pub struct Parameters {
    test: String
}
#[derive(Debug)]
pub struct Flow
{
    name: String,
    version: String,
    // task_runner: ConcurrentTaskRunner,
    description: String,
    timeout_seconds: Option<f32>,
    validate_parameters: bool,
}

// impl<F,P,R> Flow<F,P,R> {
//     fn validate_parameters(self, param: Parameters) -> Parameters;
//     fn serialize_parameters(self, param: Parameters) -> Parameters;
// }


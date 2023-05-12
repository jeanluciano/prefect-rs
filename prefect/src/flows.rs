
#[derive(Debug)]
pub struct Flow
{
    name: String,
    version: String,
    task_runner: ConcurrentTaskRunner,
    description: String,
    timeout_seconds: Option<f32>,
    validate_parameters: bool,
}

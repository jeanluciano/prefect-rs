// struct DeployementSpec{

//     name: String,
//     flow: Flow,
//     flow_name: String,
//     flow_location: String,
//     flow_storage: Optional<Union<StorageBlock, UUID>>,
//     parameters: Parameters,
//     schedule: SCHEDULE_TYPES,
//     tags: Vec<String>,
//     flow_runner: Union<FlowRunner, FlowRunnerSettings>,

// }

// trait Validate {
//     fn ensure_paths_are_absolute_strings(cls, value);
//     fn validate(self, client:OrionClient);
//     fn create_deployement(self, client: OrionClient, validate: bool) -> Bytes;

// }


// fn select_flow(flows: Vec<Flow>, flow_name: String, from_message:String) -> Flow;
// fn select_deployment(
//     deployments: Iterable<DeploymentSpec>,
//     deployment_name:String,
//     flow_name:String,
//     from_message: String,
// ) -> Flow;

// fn load_flows_from_script(path: str) -> Set<Flow>;
// fn load_flow_from_script(path: str, flow_name: String) -> Flow;
// fn deployment_specs_and_flows_from_script(
//     script_path: str,
// ) -> Tuple<Dict<DeploymentSpec, str>, Set<Flow>>;
// fn deployment_specs_from_script(path: str) -> Dict<DeploymentSpec, str>;

// fn deployment_specs_from_yaml(path: str) -> Dict<DeploymentSpec, dict>;

// trait context {
//     fn register_new_specs();
//     fn register_spec(spec: DeployementSpec);
// }

// load_flow_from_text(script_contents: AnyStr, flow_name: str);
// load_flow_from_deployment(
//     deployment: schemas.core.Deployment, client: OrionClient
// ) -> Flow;
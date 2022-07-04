use uuid::Uuid;
extern crate reqwest;

struct Flow {}

pub struct OrionClient {
    // api:API,
    api_key: String,
    api_version: str,
    // http_settings: http,
    _client: reqwest::Client
}

// impl OrionClient { 
//     fn api_url(&self){
//         return "client.baseurl"
//     }

//     fn api_healthcheck(&self){

//     }

//     fn create_flow(&self, flow: Flow) -> Bytes{
//         let mut map = HashMap::new();
//         map.insert("lang", "rust");
//         response = self._client.post("/flows")
//         .json(&map)
//         .send()
//         .await?;
//         let flow_id = response.id;

//         return uuid!(flow_id)
//     }
//     fn read_flow(&self, flow_id:Bytes){
//         response = self._client.post("/flows/{flow_id}").await?;
//         return response
//     }

//     fn read_flows(
//         &self,
//         flow_filter: FlowFilter,
//         flow_run_filter: FlowRunFilter,
//         task_run_filter: DeploymentFilter,
//         limit: i32,
//         offset: i32
//     ){
//     let mut body = HashMap::new();
//     body.insert("flows", flow_filter);
//     body.insert("flow_runs", flow_run_filter);
//     response = self._client.post("/flows/filter")
//     .json(&body)
//     .await?;
//     return response
//     }

//     fn read_flow_by_name(&self, flow_name:string){
//         response = self._client.get("/flows/name/{flow_name}")
//         .send()
//         .await?;
//         return response
     
//     }
//     fn create_flow_run_from_deployment(
//         &self,
//         deployment_id: Bytes,
//         parameters: Parameters,
//         context: Context,
//         state: State,
//         flow_runner: String,
//     ){
//         response = self._client.post("/deployments/{deployment_id}create_flow_run")
//         .json()
//         .send()
//         .await?;

//         return response
//     }
//     fn create_flow_run(
//         &self,
//         flow: Flow,
//         name: String,
//         parameters: Parameters,
//         context: Context,
//         tags: [String],
//         parent_task_run_id: Bytes,
//         state: State,
//         flow_runner: String

//     ) {
//         let response = self._client.post("/flow_runs/")
//         .json()
//         .send()
//         .await?;

//         return response
//     }
//     fn update_flow_run(
//         &self,
//         flow_run_id: Bytes,
//         flow_version: String,
//         parameters: Parameters,
//         name: String
//     ) {
//         let response = self._client.patch("/flow_runs/{flow_run_id}")
//         .json()
//         .send()
//         .await?;
//         return response
//     }
//     fn create_concurrncy_limit(
//         &self,
//         tag: String,
//         concurrency_limit: i32,
//     ){
//         let response = self._client.post("/concurrency_limits/")
//         .json()
//         .send()
//         .await?;
        
//         let concurrency_limit_id = response.id;
//         return uuid!(concurrency_limit_id)

//     }
//     fn read_concurrency_limit_by_tag(
//         &self,
//         tag: String,
//     ){
//         response = self._client.get("/concurrency_limits/tag/{tag}").send();
//         return response
//     }
//     fn read_concurrency_limits(
//         &self,
//         limit: i32,
//         offset: i32,
//     ){
//         response = self._client.get("/concurrency_limits/filter")
//         .json()
//         .send()
//         .await?;
//         return response
//     }
//     fn delete_concurrency_limit_by_tag(
//         &self,
//         tag: String,
//     ){
//         let response = self._client.delete("/concurrency_limits/tag/{tag}")
//         .send()
//         .await?;
//     }
//     fn create_work_queue(
//         &self,
//         name: String,
//         tags: [String],
//         deployment_ids: [Bytes],
//         flow_runner_types: [Strings]
//     ){
//         let response = self._client.post("work_queues")
//         .json()
//         .send()
//         .await?;

//         return uuid!(response.id)
//     }
//     fn read_work_queue_by_name(
//         &self,
//         name: String,
//     ){
//         let response = self._client.get("/work_queues/name/{name}")
//         .send()
//         .await?;

//         return "get /work_queues/name/{name}"
//     }
//     fn update_work_queue(
//         &self,
//         id: Bytes,

//     ){
//         let response = self._client.patch("/work_queues/{id}")
//         .json()
//         .send()
//         .await?;
//     }

//     fn get_runs_in_work_queue(
//         &self,
//         id: Bytes,
//         limit: i32,
//         scheduled_before: Datetime
//     ){
//         let response = self._client.post("/work_queues/{id}/get_runs")
//         .json()
//         .send()
//         .await?;
        
//         return response
//     }

//     fn read_work_queue(
//         &self,
//         id:Bytes
//     ){
//         let response = self._client.get("/work_queues/{id}")
//         .send()
//         .await?;

//         return response
//     }
//     fn read_work_queues(
//         &self,
//         limit: i32,
//         offset: i32
//     ){
//         let response = self._client.post("/work_queues/{id}/get_runs")
//         .json()
//         .send()
//         .await?;

//         return response
//     }

//     fn delete_work_queue_by_id(
//         &self,
//         id: Bytes
//     ){
//         let response = self._client.delete("/work_queues/{id}")
//         .send()
//         .await?;
//         return "delete /work_queues/{id}"
//     }

//     fn create_block(
//         &self,
//         block: Block,
//         block_spec_id: Bytes,
//         name: String,
//     ){
//         let response = self._client.post("/work_queues/{id}/get_runs")
//         .json()
//         .send()
//         .await?;
//         return uuid!(response.id)
//     }
//     fn read_block_spec_by_name(
//         &self,
//         name: String,
//         version: String
//     ){
//         let response = self._client.get("/work_queues/{id}")
//         .send()
//         .await?;

//         return response
//     }
//     fn read_block_specs(
//         &self,
//         block_type: String
//     ){
//         let response = self._client.post("/work_queues/{id}/get_runs")
//         .json()
//         .send()
//         .await?;
//         return response
//     }
//     fn read_block(
//         &self,
//         block_id: Bytes
//     ){
//         let response = self._client.get("/work_queues/{id}")
//         .send()
//         .await?;
//         return response
//     }
//     fn read_block_by_name(
//         &self,
//         name: String,
//         block_spec_name: String,
//         block_spec_version: String
//     ){
//         let response = self._client.get("/block_specs/{block_spec_name}/versions/{block_spec_version}/block/{name}")
//         .send()
//         .await?;
//         return response
   
//     }
//     fn read_blocks(
//         &self,
//         block_spec_type: str ,
//         offset: i32,
//         limit: i32,
//         as_json: bool,
//     ){
//         let response = self._client.post("/blocks/filter")
//         .json()
//         .send()
//         .await?;

//         return response
//     }
//     fn create_deployment(
//         &self,
//         flow_id: Bytes,
//         name: String,
//         flow_data: DataDocument,
//         schedule: Schedule,
//         parameters: Parameters,
//         tags: [String],
//         flow_runner: String
//     ){
//         let response = self._client.post("/deployments/")
//         .json()
//         .send()
//         .await?;
//         return uuid!(response.ip)
//     }
//     fn read_deployment(
//         &self,
//         deployment_id: Bytes
//     ){
//         let response = self._client.get("/deployments/{deployment_id}")
//         .send()
//         .await?;
//         return response
//     }
//     fn read_deployment_by_name(
//         &self,
//         name: String,
//     ){
//         let response = self._client.get("/deployments/name/{name}")
//         .send()
//         .await?;
//         return response
    
//     }
//     fn read_deployments(
//         &self,
//         flow_filter: FlowFilter,
//         flow_run_filter: FlowRunFilter,
//         task_run_filter: TaskRunFilter,
//         deployment_filter: DeploymentFilter,
//         limit: i32,
//         offset: i32

//     ){
//         let response = self._client.post("/deployments/filter")
//         .json()
//         .send()
//         .await?;
//         return response
//     }
//     fn delete_deployment(
//         &self,
//         deployment_id: Bytes
//     ){
//         let response = self._client.delete("/deployments/{deployment_id}")
//         .send()
//         .await?;

//     }
//     fn read_flow_run(
//         &self,
//         flow_run_id: Bytes
//     ){
//         let response = self._client.get("/flow_runs/{flow_run_id}")
//         .send()
//         .await?;
//         return response
 
//     }

//     fn read_flow_runs(
//         &self,
//         flow_filter: FlowFilter,
//         flow_run_filter: FlowRunFilter,
//         task_run_filter: TaskRunFilter,
//         deployment_filter: DeploymentFilter,
//         sort: FlowRunSort,
//         limit: i32,
//         offset: i32
//     ){
//         let response = self._client.post("/flow_runs/filter")
//         .json()
//         .send()
//         .await?;
//         return response

//     }
//     fn get_default_storage_block(&self, as_json:bool){
//         let response = self._client.post("/blocks/get_default_storage_block")
//         .json()
//         .send()
//         .await?;
//         return response

//     }
//     fn set_default_storage_block(self, block_id:Bytes){
//         let response = self._client.post("/blocks/{block_id}/set_default_storage_block")
//         .json()
//         .send()
//         .await?;

//     }
//     fn clear_default_storage_block(&self){
//         let response = self._client.post("/blocks/clear_default_storage_block")
//         .json()
//         .send()
//         .await?;
//     }
//     fn persit_data(&self){
//         return "datadocment sqlite"
//     }
//     fn retrieve_data(&self){
//         return "datadocument s"
//     }
//     fn persist_object(&self){
//         return "datadocument"
//     }
//     fn retrieve_object(&self){
//         return "datadocument"
//     }
//     fn set_flow_run_state(
//         &self,
//         flow_run_id: Bytes,
//         state: State,
//         force: bool,
//         backend_state_data: DataDocument
//     ){
//         let response = self._client.post("/flow_runs/{flow_run_id}/set_state")
//         .json()
//         .send()
//         .await?;
//         return response
//     }
//     fn  read_flow_run_states(
//         &self,
//         flow_run_id: Bytes
//     ){
//         let response = self._client.get("/flow_run_states/")
//         .json()
//         .send();
//         return response
//     }
//     fn create_task_run(
//         &self,
//         task: Task,
//         flow_run_id: Bytes,
//         dynamic_key: String,
//         name: String,
//         extra_tags:[String],
//         state: State,
//         task_inputs: TaskInputs
//     ){
//         let response = self._client.post("/task_runs/")
//         .json()
//         .send()
//         .await?;
//         return response
//     }
//     fn read_task_run(&self, task_run_id: Bytes){
//         let response = self._client.get("/flow_run_states/")
//         .json()
//         .send();
//         return response
//     }

//     fn read_task_runs(
//         &self,
//         flow_filter: FlowFilter,
//         flow_run_filter: FlowRunFilter,
//         task_run_filter: TaskRunFilter,
//         deployment_filter: DeploymentFilter,
//         sort: FlowRunSort,
//         limit: i32,
//         offset: i32
//     ){
//         let response = self._client.post("/task_runs/filter")
//         .json()
//         .send()
//         .await?;
//         return response

//     }
//     fn propose_state(
//         &self,
//         state: State,
//         backend_state_data: DataDocument,
//         task_run_id: Bytes,
//         flow_run_id: Bytes
//     ){
//         return "block storage"
//     }
//     fn set_task_run_state(
//         &self,
//         task_run_id: Bytes,
//         state: State, 
//         force:bool,
//         backend_state_data: DataDocument
//     ){
//         let response = self._client.post("/task_runs/{task_run_id}/set_state")
//         .json()
//         .send()
//         .await?;
//         return response

//     }
//     fn read_task_run_states(&self,task_run_id:Bytes){
//         let response = self._client.get("/task_run_states/")
//         .json()
//         .send();
//         return response
 
//     }
//     fn create_logs(&self, logs: Logs){
//         let response = self._client.post("/logs/")
//         .json()
//         .send()
//         .await?;
    
        
//     }
//     fn read_logs(&self){
//         let response = self._client.post("/logs/filter")
//         .json()
//         .send()
//         .await?;
//         return response
        
//     }
//     fn resolve_datadoc(&self){
//         return "datadoc"
//     }
//     fn _aenter(&self){
//         return self
//     }
// }


// struct Flow{}
// struct API {}
// struct http {}

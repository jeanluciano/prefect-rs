use crate::types::{
    core::{Flow, FlowRun},
    states::State,
};
use reqwest::Error;
use std::collections::HashMap;
use uuid::{Uuid};
extern crate reqwest;

pub struct OrionClient {
    // api:API,
    api_key: String,
    api_version: String,
    // http_settings: http,
    _client: reqwest::Client,
}

impl OrionClient {
    fn api_url(&self) -> &str {
        return "client.baseurl";
    }

    fn api_healthcheck(&self) {}

    async fn create_flow(&self, flow: &Flow) -> Result<Uuid, Error> {
        let mut map = HashMap::new();
        map.insert("lang", "rust");
        let response = self
            ._client
            .post("/flows")
            .json(&map)
            .send()
            .await?
            .json::<Flow>()
            .await?;

        return Ok(response.id);
    }

    async fn read_flow(&self, flow_id: Uuid) -> Result<Flow, Error> {
        let response = self
            ._client
            .get(format!("/flows/{}", flow_id))
            .send()
            .await?
            .json::<Flow>()
            .await?;
        return Ok(response);
    }

    // async fn read_flows(
    //     &self,
    //     flow_filter: FlowFilter,
    //     flow_run_filter: FlowRunFilter,
    //     task_run_filter: DeploymentFilter,
    //     limit: i32,
    //     offset: i32,
    // ) -> Result<Flow, Error> {
    //     let mut body = HashMap::new();
    //     body.insert("flows", flow_filter);
    //     body.insert("flow_runs", flow_run_filter);
    //     let response = self._client
    //     .post("/flows/filter")
    //     .json(&body)
    //     .send()
    //     .await?;
    //     return Ok(response);
    // }

    async fn read_flow_by_name(&self, flow_name: String) -> Result<Flow, Error> {
        let response = self
            ._client
            .get(format!("/flows/name/{}", flow_name))
            .send()
            .await?
            .json::<Flow>()
            .await?;
        return Ok(response);
    }
    // fn create_flow_run_from_deployment(
    //     &self,
    //     deployment_id: Uuid,
    //     parameters: Parameters,
    //     context: Context,
    //     state: State,
    //     flow_runner: String,
    // ){
    //     response = self._client.post("/deployments/{deployment_id}create_flow_run")
    //     .json()
    //     .send()
    //     .await?;

    //     return response
    // }
    async fn create_flow_run(
        &self,
        flow: &Flow,
        name: String,
        parameters: HashMap<String, String>,
        context: HashMap<String, String>,
        tags: Vec<String>,
        parent_task_run_id: Uuid,
        state: State,
        flow_runner: String,
    ) -> Result<FlowRun, Error> {
        let flow_id = self.create_flow(flow).await?;
        let flow_run_create = FlowRun::new(
            name,
            flow_id,
            flow.version.to_string(),
            parameters,
            tags,
            state,
            // flow_runner,
            context,
            parent_task_run_id,
        );

        let response = self
            ._client
            .post("/flow_runs/")
            .json(&flow_run_create)
            .send()
            .await?
            .json::<FlowRun>()
            .await?;

        return Ok(response);
    }
    // fn update_flow_run(
    //     &self,
    //     flow_run_id: Uuid,
    //     flow_version: String,
    //     parameters: Parameters,
    //     name: String
    // ) {
    //     let response = self._client.patch("/flow_runs/{flow_run_id}")
    //     .json()
    //     .send()
    //     .await?;
    //     return response
    // }
    // fn create_concurrncy_limit(
    //     &self,
    //     tag: String,
    //     concurrency_limit: i32,
    // ){
    //     let response = self._client.post("/concurrency_limits/")
    //     .json()
    //     .send()
    //     .await?;

    //     let concurrency_limit_id = response.id;
    //     return uuid!(concurrency_limit_id)

    // }
    // fn read_concurrency_limit_by_tag(
    //     &self,
    //     tag: String,
    // ){
    //     response = self._client.get("/concurrency_limits/tag/{tag}").send();
    //     return response
    // }
    // fn read_concurrency_limits(
    //     &self,
    //     limit: i32,
    //     offset: i32,
    // ){
    //     response = self._client.get("/concurrency_limits/filter")
    //     .json()
    //     .send()
    //     .await?;
    //     return response
    // }
    // fn delete_concurrency_limit_by_tag(
    //     &self,
    //     tag: String,
    // ){
    //     let response = self._client.delete("/concurrency_limits/tag/{tag}")
    //     .send()
    //     .await?;
    // }
    // fn create_work_queue(
    //     &self,
    //     name: String,
    //     tags: [String],
    //     deployment_ids: [Uuid],
    //     flow_runner_types: [Strings]
    // ){
    //     let response = self._client.post("work_queues")
    //     .json()
    //     .send()
    //     .await?;

    //     return uuid!(response.id)
    // }
    // fn read_work_queue_by_name(
    //     &self,
    //     name: String,
    // ){
    //     let response = self._client.get("/work_queues/name/{name}")
    //     .send()
    //     .await?;

    //     return "get /work_queues/name/{name}"
    // }
    // fn update_work_queue(
    //     &self,
    //     id: Uuid,

    // ){
    //     let response = self._client.patch("/work_queues/{id}")
    //     .json()
    //     .send()
    //     .await?;
    // }

    // fn get_runs_in_work_queue(
    //     &self,
    //     id: Uuid,
    //     limit: i32,
    //     scheduled_before: Datetime
    // ){
    //     let response = self._client.post("/work_queues/{id}/get_runs")
    //     .json()
    //     .send()
    //     .await?;

    //     return response
    // }

    // fn read_work_queue(
    //     &self,
    //     id:Uuid
    // ){
    //     let response = self._client.get("/work_queues/{id}")
    //     .send()
    //     .await?;

    //     return response
    // }
    // fn read_work_queues(
    //     &self,
    //     limit: i32,
    //     offset: i32
    // ){
    //     let response = self._client.post("/work_queues/{id}/get_runs")
    //     .json()
    //     .send()
    //     .await?;

    //     return response
    // }

    // fn delete_work_queue_by_id(
    //     &self,
    //     id: Uuid
    // ){
    //     let response = self._client.delete("/work_queues/{id}")
    //     .send()
    //     .await?;
    //     return "delete /work_queues/{id}"
    // }

    // fn create_block(
    //     &self,
    //     block: Block,
    //     block_spec_id: Uuid,
    //     name: String,
    // ){
    //     let response = self._client.post("/work_queues/{id}/get_runs")
    //     .json()
    //     .send()
    //     .await?;
    //     return uuid!(response.id)
    // }
    // fn read_block_spec_by_name(
    //     &self,
    //     name: String,
    //     version: String
    // ){
    //     let response = self._client.get("/work_queues/{id}")
    //     .send()
    //     .await?;

    //     return response
    // }
    // fn read_block_specs(
    //     &self,
    //     block_type: String
    // ){
    //     let response = self._client.post("/work_queues/{id}/get_runs")
    //     .json()
    //     .send()
    //     .await?;
    //     return response
    // }
    // fn read_block(
    //     &self,
    //     block_id: Uuid
    // ){
    //     let response = self._client.get("/work_queues/{id}")
    //     .send()
    //     .await?;
    //     return response
    // }
    // fn read_block_by_name(
    //     &self,
    //     name: String,
    //     block_spec_name: String,
    //     block_spec_version: String
    // ){
    //     let response = self._client.get("/block_specs/{block_spec_name}/versions/{block_spec_version}/block/{name}")
    //     .send()
    //     .await?;
    //     return response

    // }
    // fn read_blocks(
    //     &self,
    //     block_spec_type: str ,
    //     offset: i32,
    //     limit: i32,
    //     as_json: bool,
    // ){
    //     let response = self._client.post("/blocks/filter")
    //     .json()
    //     .send()
    //     .await?;

    //     return response
    // }
    // fn create_deployment(
    //     &self,
    //     flow_id: Uuid,
    //     name: String,
    //     flow_data: DataDocument,
    //     schedule: Schedule,
    //     parameters: Parameters,
    //     tags: [String],
    //     flow_runner: String
    // ){
    //     let response = self._client.post("/deployments/")
    //     .json()
    //     .send()
    //     .await?;
    //     return uuid!(response.ip)
    // }
    // fn read_deployment(
    //     &self,
    //     deployment_id: Uuid
    // ){
    //     let response = self._client.get("/deployments/{deployment_id}")
    //     .send()
    //     .await?;
    //     return response
    // }
    // fn read_deployment_by_name(
    //     &self,
    //     name: String,
    // ){
    //     let response = self._client.get("/deployments/name/{name}")
    //     .send()
    //     .await?;
    //     return response

    // }
    // fn read_deployments(
    //     &self,
    //     flow_filter: FlowFilter,
    //     flow_run_filter: FlowRunFilter,
    //     task_run_filter: TaskRunFilter,
    //     deployment_filter: DeploymentFilter,
    //     limit: i32,
    //     offset: i32

    // ){
    //     let response = self._client.post("/deployments/filter")
    //     .json()
    //     .send()
    //     .await?;
    //     return response
    // }
    // fn delete_deployment(
    //     &self,
    //     deployment_id: Uuid
    // ){
    //     let response = self._client.delete("/deployments/{deployment_id}")
    //     .send()
    //     .await?;

    // }
    // fn read_flow_run(
    //     &self,
    //     flow_run_id: Uuid
    // ){
    //     let response = self._client.get("/flow_runs/{flow_run_id}")
    //     .send()
    //     .await?;
    //     return response

    // }

    // fn read_flow_runs(
    //     &self,
    //     flow_filter: FlowFilter,
    //     flow_run_filter: FlowRunFilter,
    //     task_run_filter: TaskRunFilter,
    //     deployment_filter: DeploymentFilter,
    //     sort: FlowRunSort,
    //     limit: i32,
    //     offset: i32
    // ){
    //     let response = self._client.post("/flow_runs/filter")
    //     .json()
    //     .send()
    //     .await?;
    //     return response

    // }
    // fn get_default_storage_block(&self, as_json:bool){
    //     let response = self._client.post("/blocks/get_default_storage_block")
    //     .json()
    //     .send()
    //     .await?;
    //     return response

    // }
    // fn set_default_storage_block(self, block_id:Uuid){
    //     let response = self._client.post("/blocks/{block_id}/set_default_storage_block")
    //     .json()
    //     .send()
    //     .await?;

    // }
    // fn clear_default_storage_block(&self){
    //     let response = self._client.post("/blocks/clear_default_storage_block")
    //     .json()
    //     .send()
    //     .await?;
    // }
    // fn persit_data(&self){
    //     return "datadocment sqlite"
    // }
    // fn retrieve_data(&self){
    //     return "datadocument s"
    // }
    // fn persist_object(&self){
    //     return "datadocument"
    // }
    // fn retrieve_object(&self){
    //     return "datadocument"
    // }
    // fn set_flow_run_state(
    //     &self,
    //     flow_run_id: Uuid,
    //     state: State,
    //     force: bool,
    //     backend_state_data: DataDocument
    // ){
    //     let response = self._client.post("/flow_runs/{flow_run_id}/set_state")
    //     .json()
    //     .send()
    //     .await?;
    //     return response
    // }
    // fn  read_flow_run_states(
    //     &self,
    //     flow_run_id: Uuid
    // ){
    //     let response = self._client.get("/flow_run_states/")
    //     .json()
    //     .send();
    //     return response
    // }
    // fn create_task_run(
    //     &self,
    //     task: Task,
    //     flow_run_id: Uuid,
    //     dynamic_key: String,
    //     name: String,
    //     extra_tags:[String],
    //     state: State,
    //     task_inputs: TaskInputs
    // ){
    //     let response = self._client.post("/task_runs/")
    //     .json()
    //     .send()
    //     .await?;
    //     return response
    // }
    // fn read_task_run(&self, task_run_id: Uuid){
    //     let response = self._client.get("/flow_run_states/")
    //     .json()
    //     .send();
    //     return response
    // }

    // fn read_task_runs(
    //     &self,
    //     flow_filter: FlowFilter,
    //     flow_run_filter: FlowRunFilter,
    //     task_run_filter: TaskRunFilter,
    //     deployment_filter: DeploymentFilter,
    //     sort: FlowRunSort,
    //     limit: i32,
    //     offset: i32
    // ){
    //     let response = self._client.post("/task_runs/filter")
    //     .json()
    //     .send()
    //     .await?;
    //     return response

    // }
    // fn propose_state(
    //     &self,
    //     state: State,
    //     backend_state_data: DataDocument,
    //     task_run_id: Uuid,
    //     flow_run_id: Uuid
    // ){
    //     return "block storage"
    // }
    // fn set_task_run_state(
    //     &self,
    //     task_run_id: Uuid,
    //     state: State,
    //     force:bool,
    //     backend_state_data: DataDocument
    // ){
    //     let response = self._client.post("/task_runs/{task_run_id}/set_state")
    //     .json()
    //     .send()
    //     .await?;
    //     return response

    // }
    // fn read_task_run_states(&self,task_run_id:Uuid){
    //     let response = self._client.get("/task_run_states/")
    //     .json()
    //     .send();
    //     return response

    // }
    // fn create_logs(&self, logs: Logs){
    //     let response = self._client.post("/logs/")
    //     .json()
    //     .send()
    //     .await?;

    // }
    // fn read_logs(&self){
    //     let response = self._client.post("/logs/filter")
    //     .json()
    //     .send()
    //     .await?;
    //     return response

    // }
    // fn resolve_datadoc(&self){
    //     return "datadoc"
    // }
    // fn _aenter(&self){
    //     return self
    // }
}

// struct Flow{}
// struct API {}
// struct http {}

// use crate::context::FlowRunContext;
// use crate::flows::{Flow, Parameters};
// use crate::client::OrionClient;
// use crate::futures::DedicatedExecutor;
// use crate::types::states::State;
// use crate::types::core::{FlowRun};
// use chrono::Utc;
// use std::collections::hash_set::Union;
// use std::future::Future;
// use std::sync::Arc;
// use tokio::sync::RwLock;

// pub fn enter_flow_run_engine_from_flow_call<F>(
//     flow: Flow<F>,
//     parameters: Parameters,
//     client: OrionClient,
// ) -> Union<State, Future> {
//     return create_then_begin_flow_run(flow, parameters,client);
// }

// // pub fn enter_flow_run_engine_from_subprocess(flow_run_id: Bytes) -> State {}
// pub async fn create_then_begin_flow_run<F>(
//     flow: Flow<F>,
//     parameters: Parameters,
//     client: OrionClient,
// ) -> State {
//     // let connect_error = await client.api_healthcheck()
//     // if connect_error:
//     // raise RuntimeError(
//     //     f"Cannot create flow run. Failed to reach API at {client.api_url}."
//     // ) from connect_error
//     // state = Pending();

//     // let flow_run = client.create_flow_run(
//     //     flow,
//     //     parameters=flow.serialize_parameters(parameters),
//     //     state=state,
//     //     tags=TagsContext.get().current_tags,
//     // ).await;
//     return begin_flow_run(flow, flow_run, parameters, client);
// }
// // fn retrieve_flow_then_begin_flow_run(flow_run_id: UUID, client: OrionClient) -> State;
// async fn begin_flow_run<F>(
//     flow: Flow<F>,
//     flow_run: FlowRun<F>,
//     parameters: Parameters,
//     client: OrionClient,
// ) -> State {
//     const  mut flow_run_context:FlowRunContext<F> = FlowRunContext {
//         flow,
//         flow_run,
//         task_runner: flow.task_runner.start(),
//         client,
//         start_time: Utc::now(),
//     };

//     let shared_context: Arc::new<RwLock::new<flow_run_context>>;
//     let terminal_state =
//         orchestrate_flow_run(flow, flow_run, parameters, client, partial_flow_run_context).await;

//     terminal_state
// }

// // fn create_and_begin_subflow_run(flow: Flow, parameters: Parameters, client: OrionClient) -> State;
// fn orchestrate_flow_run(
//     flow: Flow,
//     flow_run: FlowRun,
//     parameters: Parameters,
//     client: OrionClient,
//     partial_flow_run_context: PartialModel<FlowRunContext>,
// ) -> State;

// fn enter_task_run_engine(
//     task: Task,
//     parameters: Parameters,
//     dynamic_key: str,
//     wait_for: Optional<Iterable<PrefectFuture>>,
// ) -> Union<PrefectFuture, Awaitable<PrefectFuture>> {

//     // let begin_run = create_and_begin_subflow_run(
//     //     task,
//     //     flow_run_context=flow_run_context,
//     //     parameters=parameters,
//     //     dynamic_key=dynamic_key,
//     //     wait_for=wait_for,
//     // );

//     // if task.isasync && flow_run_context.flow.isasync {
//     //     return begin_run()
//     // } else if flow_run_context.flow.isasync {
//     //     return run_async_from_worker_thread(begin_run)
//     // } else {
//     //     return flow_run_context.sync_portal.call(begin_run)
//     // }
// }

// fn collect_task_run_inputs(expr: Any) -> Set<Union<TaskRunResult, Parameter, Constant>>;

// fn create_and_submit_task_run(
//     task: Task,
//     flow_run_context: FlowRunContext,
//     parameters: Parameter,
//     dynamic_key: str,
//     wait_for: Optional<Iterable<prefectFuture>>,
// ) -> PrefectFuture {
//     let task_run = flow_run_context
//         .client
//         .create_task_run(
//             task = task,
//             flow_run_id = flow_run_context.flow_run.id,
//             dynamic_key = dynamic_key,
//             state = Pending(),
//             extra_tags = TagsContext.get().current_tags,
//             task_inputs = task_inputs,
//         )
//         .await;

//     let future = flow_run_context
//         .task_runner
//         .submit(
//             task_run,
//             run_fn = begin_task_run,
//             run_kwargs = dict(
//                 task = task,
//                 task_run = task_run,
//                 parameters = parameters,
//                 wait_for = wait_for,
//                 result_storage = flow_run_context.result_storage,
//                 settings = prefect.context.SettingsContext.get().copy(),
//             ),
//             asynchronous = task.isasync && flow_run_context.flow.isasync,
//         )
//         .await;
//     flow_run_context.task_run_futures.append(future);

//     future
// }

// fn begin_task_run(
//     task: Task,
//     task_run: TaskRun,
//     parameters: Parameter,
//     wait_for: Optional<Iterable<PrefectFuture>>,
//     result_storage: StorageBlock,
//     settings: SettingsContext,
// );

// fn orchestrate_task_run(
//     task: Task,
//     task_run: TaskRun,
//     parameters: Parameter,
//     wait_for: Optional<Iterable<PrefectFuture>>,
//     result_storage: StorageBlock,
//     client: OrionClient,
// ) -> State;

// fn wait_for_task_runs_and_report_crashes(
//     task_run_futures: Iterable<PrefectFuture>,
//     client: OrionClient,
// ) -> None;

// fn report_flow_run_crashes(flow_run: FlowRun, client: OrionClient);

// fn resolve_inputs(parameters: Parameters, return_data: bool) -> Parameters;

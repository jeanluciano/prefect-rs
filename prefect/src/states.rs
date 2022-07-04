// fn exception_to_crashed_state(exc: BaseException) -> State;
// fn safe_encode_exception(exception: BaseException) -> BaseException;
// fn return_value_to_state(result: Any, serializer: Cloudpickle) -> State;
// fn raise_failed_state(state: State, client: OrionClient) -> None;
// fn is_state(obj: any) -> Bool;

use crate::types::states::State;

struct StateGroup {
    states: Vec<State>,
    type_counts: i32,
    total_count: i32,
    not_final_count: i32
}

trait GetStates {
    fn fail_count(self);
    fn all_completed(self);
    fn all_final(self);
    fn counts_messages(self);
}
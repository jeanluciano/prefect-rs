use prefect_macro::*;
use prefect::futures::DedicatedExecutor;
use prefect::context::FlowRunContext
#[test]
fn run_test() {
    #[task]
    fn my_task(tester: i32, testy: i64) -> i32 {
        return 42;
    }

    #[flow]
    fn my_flow() -> i32 {
        let test_flow = my_task(33,33);
    }

    let test_task = my_flow();
    
    assert_eq!(test_task, ())
}


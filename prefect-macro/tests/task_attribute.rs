#![feature(unboxed_closures, fn_traits)]

use prefect_macro::*;
// struct Task<F>
// {
//     task_fn: Box<F>,
//     name: String,
//     // description: String,
//     // tags: Vec<String>,
//     // cache_key_fn: fn(str) -> str,
//     // cache_expiration: Datetime,
//     // retries: i32,
//     // retry_delay_seconds: f32,
//     // isasync: bool,
//     // task_key: String,
//     // _dynamic_key: i32,
// }

// struct Flow<F> {
//     callable:F,
//     name: String
// }
pub mod futures;

#[test]
fn run_test() {
    #[task]
    fn my_task(tester: i32, testy: i64) -> i32 {
        return 42;
    }

    #[flow]
    fn my_flow(tester: i32, testy: i64) -> i32 {
        let test_flow = my_task(33,33);
    }

    let test_task = my_flow(33,333);
    println!("{}", test_task);
    assert_eq!(test_task, 42)
}


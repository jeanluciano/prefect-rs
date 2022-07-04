use chrono::{DateTime, Utc};

pub struct Task<F> {
    task_fn: F,
    name: String,
    // description: String,
    // tags: Vec<String>,
    // cache_key_fn: fn(str) -> str,
    // cache_expiration: DateTime<Utc>,
    // retries: i32,
    // retry_delay_seconds: f32,
    // isasync: bool,
    // task_key: String,
    // _dynamic_key: i32,
}

trait Create {
    fn new<F,P,R>(self) -> Task<F>;
    fn get_and_update_dynamic_key(self) -> str;
}

impl<F> Create for Task<F> {
    fn new(
        name: String,
        task_fn: F,
        // description: String,
        // tags: Vec<String>,
        // cache_key_fn: fn(str) -> str,
        // cache_expiration: DateTime<Utc>,
        // retries: i32,
        // retry_delay_seconds: f32,
    ) -> Task<F> {
        Task { 
            name: name,
            task_fn,
            // description,
            // tags,
            // retries,
            // cache_key_fn,
            // cache_expiration,
            // retry_delay_seconds,
            // isasync: isasynctfunction(callable),
            // task_key: stable_hash(name),
            // _dynamic_key: 0
         }
    }
    fn get_and_update_dynamic_key(& mut self) -> str {
        let current_key = self._dynamic_key;
        self._dynamic_key += 1;
        current_key
    }
}


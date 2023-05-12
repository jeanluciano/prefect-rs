use chrono::{DateTime, Utc};
use md5::compute;
type CacheFunc = fn(String) -> String;
pub struct Task {
    pub name: String,
    pub description: String,
    tags: Vec<String>,
    cache_key_fn: Option<CacheFunc>,
    cache_expiration: Option<DateTime<Utc>>,
    retries: i32,
    retry_delay_seconds: i32,
    isasync: bool,
    task_key: String,
    pub _dynamic_key: i32,
}

impl Task {
    fn new(
        self,
        name: String,
        description: String,
        tags: Option<Vec<String>>,
        cache_expiration: Option<DateTime<Utc>>,
        retries: i32,
        retry_delay_seconds: i32,
        isasync: bool
    ) -> Task {

        let tags = init_tags(tags);
        let cache_key_fn = None;

        let task_key = stable_hash(&name,tags.join(","));
        let _dynamic_key = 0;
        Self {
            name,
            description,
            tags,
            cache_expiration,
            retries,
            retry_delay_seconds,
            isasync,
            task_key,
            _dynamic_key,
            cache_key_fn
        }
    }
    fn get_and_update_dynamic_key(&mut self) -> i32 {
        let current_key = self._dynamic_key;
        self._dynamic_key += 1;
        current_key
    }
}

fn init_tags(tags: Option<Vec<String>>) -> Vec<String> {
    match tags {
        Some(x) => x,
        None => Vec::new()
    }
}

fn stable_hash(name: &str, tags: String ) -> String{
    let digest = md5::compute(format!("{}{}", name, tags));
    format!("{:x}", digest )
}
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

static SYMBOL_MAP: LazyLock<Mutex<HashMap<String, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub struct Symbol {
    name: String,
    count: usize,
}

impl Symbol {
    fn counter(s: &str) -> usize {
        let mut map = SYMBOL_MAP.lock().unwrap();
        let count = map.entry(s.to_string()).or_insert(0);
        *count += 1;
        *count
    }
    pub fn r#for(s: &str) -> Self {
        let count = Self::counter(s);
        Self {
            name: s.to_string(),
            count,
        }
    }
}

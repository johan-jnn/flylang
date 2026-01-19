use std::collections::HashMap;

/// Source - https://stackoverflow.com/a/62118975
/// Posted by trent
/// Retrieved 2026-01-19, License - CC BY-SA 4.0
pub fn get_env_hashmap() -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (key, val) in std::env::vars_os() {
        // Use pattern bindings instead of testing .is_some() followed by .unwrap()
        if let (Ok(k), Ok(v)) = (key.into_string(), val.into_string()) {
            map.insert(k, v);
        }
    }

    map
}

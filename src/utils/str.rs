use regex::{Captures, Regex, Replacer};
use std::collections::HashMap;

pub struct ReplaceByKey(pub HashMap<String, String>);
impl ReplaceByKey {
    pub fn replace(value: &str, by: HashMap<String, String>) -> String {
        let replacer = Self(by);

        Self::get_regex().replace_all(value, replacer).to_string()
    }

    fn get_regex() -> Regex {
        Regex::new(r"(?m)\$(?:(?<KEY>[\w_]+)|(?:\{(?<DELIMITED_KEY>.+?)\}))").unwrap()
    }
}
impl Replacer for ReplaceByKey {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        let key = caps
            .name("KEY")
            .unwrap_or_else(|| caps.name("DELIMITED_KEY").expect("Issue with the regex."))
            .as_str();

        if let Some(value) = self.0.get(key) {
            dst.push_str(value);
        }
    }
}

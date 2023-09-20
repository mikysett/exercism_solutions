use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), attrs: HashMap::new() }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        for attr in attrs {
            self.attrs.insert(attr.0.to_string(), attr.1.to_string());
        }
        self
    }

    pub fn attr(&self, k: &str) -> Option<&str> {
        self.attrs.get(k).map(|s| s.as_str())
    }
}
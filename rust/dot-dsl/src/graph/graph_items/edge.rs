use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct Edge {
    pub nodes: (String, String),
    pub attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(n1: &str, n2: &str) -> Self {
        Self { nodes: (n1.to_string(), n2.to_string()), attrs: HashMap::new() }
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
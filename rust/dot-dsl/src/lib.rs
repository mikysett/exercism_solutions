pub mod graph {
    pub mod graph_items;

    pub use graph_items::node::Node;
    pub use graph_items::edge::Edge;

    use std::collections::HashMap;
    use maplit::hashmap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph { nodes: vec![], edges: vec![], attrs: hashmap![] }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn node(&self, node_name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name == node_name)
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for attr in attrs {
                self.attrs.insert(attr.0.to_string(), attr.1.to_string());
            }
            self
        }
    }
}

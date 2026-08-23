pub mod graph {
    type Attrs = std::collections::HashMap<String, String>;

    pub mod graph_items {
        pub mod node {
            pub use crate::graph::Node;
        }
        pub mod edge {
            pub use crate::graph::Edge;
        }
    }

    macro_rules! attr_impl {
        ($t:ty) => {
            impl $t {
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    self
                }

                pub fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(String::as_ref)
                }
            }
        };
    }

    #[derive(Debug, Default, PartialEq, Eq, Clone)]
    pub struct Node {
        pub name: String,
        pub attrs: Attrs,
    }

    impl Node {
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                ..Default::default()
            }
        }
    }
    attr_impl!(Node);

    #[derive(Debug, Default, PartialEq, Eq, Clone)]
    pub struct Edge {
        pub node1: String,
        pub node2: String,
        pub attrs: Attrs,
    }

    impl Edge {
        pub fn new(node1: &str, node2: &str) -> Self {
            Self {
                node1: node1.to_string(),
                node2: node2.to_string(),
                ..Default::default()
            }
        }
    }
    attr_impl!(Edge);

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: Attrs,
    }

    impl Graph {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == name)
        }
    }
    attr_impl!(Graph);
}

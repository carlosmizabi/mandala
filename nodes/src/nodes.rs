use uuid::Uuid;
use std::collections::HashMap;

pub struct Node {
    pub(crate) id: u128,
    pub(crate) nodes: HashMap<u128, Node>,
}

impl Node {
    pub fn new() -> Node {
        return Self::with(HashMap::new());
    }

    pub fn with(nodes: HashMap<u128, Node>) -> Node {
        return Node {
            id: Uuid::new_v4().as_u128(),
            nodes,
        };
    }
}

pub struct ListValue {
    node: Node,
}

impl Nodal for ListValue {
    fn get_node(&self) -> &Node {
        &self.node
    }
}

pub trait Nodal {
    fn get_node(&self) -> &Node;
    fn get_id(&self) -> &u128 {
        return &self.get_node().id;
    }
    fn get_nodes(&self) -> &HashMap<u128, Node> {
        return &self.get_node().nodes;
    }
}

pub trait Valuable {
    type Value;
    fn get_value(&self) -> &Self::Value;
}

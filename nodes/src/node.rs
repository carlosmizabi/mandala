use std::collections::HashMap;
use uuid::Uuid;

pub struct Node {
    pub id: u128,
    pub nodes: HashMap<u128, Node>,
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
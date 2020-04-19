use std::collections::HashMap;
use crate::node::Node;

pub trait ValuableNode {

    type Value;

    fn get_node(&self) -> &Node;

    fn get_value(&self) -> &Self::Value;

    fn get_id(&self) -> &u128 {
        return &self.get_node().id;
    }

    fn get_nodes(&self) -> &HashMap<u128, Node> {
        return &self.get_node().nodes;
    }
}

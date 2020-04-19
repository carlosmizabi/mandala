use std::collections::HashMap;
use crate::valuable_node::ValuableNode;
use crate::node::Node;

pub struct BooleanValue {
    value: Option<bool>,
    node: Node,
}

impl BooleanValue {
    pub fn new(value: bool, node: Node) -> BooleanValue {
        BooleanValue { value: Some(value), node }
    }
    pub fn from_value(value: bool) -> BooleanValue {
        BooleanValue { value: Some(value), node: Node::new() }
    }
}

impl ValuableNode for BooleanValue {
    type Value = Option<bool>;

    fn get_node(&self) -> &Node {
        return &self.node;
    }
    fn get_value(&self) -> &Self::Value {
        return &self.value;
    }
}


#[cfg(test)]
mod should {
    use crate::values::boolean_value::BooleanValue;

    #[test]
    fn return_the_boolean_value() {
        let cases = hashmap! {
            true => BooleanValue::from_value(true),
            false => BooleanValue::from_value(false)
        };
        for case in cases {
            assert_eq!(case.0, case.1.value.unwrap())
        }
    }
}
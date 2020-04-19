use std::collections::HashMap;
use crate::nodes::{Node, Nodal, Valuable};

pub struct BooleanValue {
    value: bool,
    node: Node,
}

impl BooleanValue {
    pub fn new(value: bool, node: Node) -> BooleanValue {
        BooleanValue { value, node }
    }

    pub fn from_value(value: bool) -> BooleanValue {
        BooleanValue { value, node: Node::new() }
    }
}

impl Nodal for BooleanValue {
    fn get_node(&self) -> &Node {
        return &self.node;
    }
}

impl Valuable for BooleanValue {
    type Value = bool;

    fn get_value(&self) -> &Self::Value {
        return &self.value;
    }
}


#[cfg(test)]
mod should {
    use crate::values::boolean_value::BooleanValue;
    use crate::nodes::{Node, Nodal, Valuable};

    #[test]
    fn return_the_boolean_value() {
        let cases = hashmap! {
            true => BooleanValue::from_value(true),
            false => BooleanValue::from_value(false)
        };
        for case in cases {
            assert_eq!(case.0, *case.1.get_value())
        }
    }
}
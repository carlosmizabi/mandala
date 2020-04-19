use std::collections::HashMap;
use crate::nodes::{Node, Nodal, Valuable};

pub struct NumberValue {
    value: i64,
    node: Node,
}

impl NumberValue {
    pub(crate) fn new(value: i64, node: Node) -> NumberValue {
        NumberValue { value, node }
    }

    pub(crate) fn from_value(value: i64) -> NumberValue {
        NumberValue { value, node: Node::new() }
    }
}


impl Valuable for NumberValue {
    type Value = i64;

    fn get_value(&self) -> &Self::Value {
        return &self.value;
    }
}

#[cfg(test)]
mod should_dj {
    use crate::nodes::boolean_node::NumberValue;
    use crate::nodes::node::{Node, Valuable};

    #[test]
    fn return_the_number_value() {
        let cases = hashmap! {
            123i32 => NumberValue::from_value(123),
            -1.23i32 => NumberValue::from_value(-1.235)
        };
        for case in cases {
            assert_eq!(case.0, *case.1.get_value())
        }
    }
}
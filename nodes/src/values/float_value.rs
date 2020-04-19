use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use crate::valuable_node::ValuableNode;
use crate::node::Node;

pub struct FloatValue {
    value: Option<f64>,
    node: Node,
}

impl FloatValue {
    pub fn new(value: f64, node: Node) -> FloatValue {
        FloatValue { value: Some(value), node }
    }

    pub fn from_value(value: f64) -> FloatValue {
        FloatValue { value: Some(value), node: Node::new() }
    }
}

impl ValuableNode for FloatValue {
    type Value = Option<f64>;

    fn get_node(&self) -> &Node { return &self.node; }

    fn get_value(&self) -> &Self::Value {
        return &self.value;
    }
}

#[cfg(test)]
mod should_dj {
    use std::collections::HashMap;
    use crate::values::float_value::FloatValue;

    #[test]
    fn return_the_number_value() {
        let mut cases= vec![
            (123f64, FloatValue::from_value(123f64)),
                (-1.235f64, FloatValue::from_value(-1.235f64))
        ];
        for case in cases {
            assert_eq!(case.0, case.1.value.unwrap())
        }
    }
}
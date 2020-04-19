use std::collections::HashMap;
use crate::nodes::{Node, Nodal, Valuable};

pub struct NumberValue {
    value: f64,
    node: Node,
}

impl NumberValue {
    pub fn new(value: f64, node: Node) -> NumberValue {
        NumberValue { value, node }
    }

    pub fn from_value(value: f64) -> NumberValue {
        NumberValue { value, node: Node::new() }
    }
}


impl Valuable for NumberValue {
    type Value = f64;

    fn get_value(&self) -> &Self::Value {
        return &self.value;
    }
}

#[cfg(test)]
mod should_dj {
    use crate::nodes::{Node, Valuable};
    use std::collections::HashMap;
    use crate::values::number_value::NumberValue;

    #[test]
    fn return_the_number_value() {
        let mut cases= vec![
            (123f64, NumberValue::from_value(123f64)),
                (-1.235f64, NumberValue::from_value(-1.235f64))
        ];
        for case in cases {
            assert_eq!(case.0, *case.1.get_value())
        }
    }
}
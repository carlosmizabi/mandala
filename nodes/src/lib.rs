#[macro_use] extern crate maplit;

pub mod nodes;
pub mod values;

#[cfg(test)]
mod tests {
    use crate::nodes::Node;

    #[test]
    fn it_works() {
        Node::new();
        assert_eq!(2 + 2, 4);
    }
}

#[macro_use] extern crate maplit;

pub mod valuable_node;
pub mod values;
pub mod node;

#[cfg(test)]
mod tests {
    use crate::node::Node;

    #[test]
    fn it_works() {
        Node::new();
        assert_eq!(2 + 2, 4);
    }
}

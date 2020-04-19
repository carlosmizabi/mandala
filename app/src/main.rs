use std::collections::HashMap;
use nodes::values::boolean_value::BooleanValue;
use nodes::valuable_node::ValuableNode;

fn main () {
    let n = BooleanValue::from_value(true);
    println!("Hello World! is it? {}", n.get_value().unwrap())
    // let x = BooleanValue::new(
    //     true,
    //     Node {
    //         id: 1,
    //         nodes: HashMap::new()
    //     });

    // let y = NumberValue::from_value(12i64);
    // print!("Value {}", y.get_value());
}


//
// #[tokio::main]
// async fn main() {
//     pretty_env_logger::init();
//     let hello = warp::path!("hello")
//         .map(|| {
//             let e = Employee {
//                 name: String::from("Carlos"),
//                 position: String::from("King")
//             };
//             warp::reply::json(&e);
//         });
//     println!("Serving on http://localhost:{}", 3030);
//     let result = warp::serve(hello)
//         .run(([127,0,0,1], 3030))
//         .await;
//     println!("Terminated");
//     return result
// }
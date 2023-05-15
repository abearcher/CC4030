mod node;

use crate::node::SellerRunner;

fn main() {
    println!("Starting first node!");
    let server_node = SellerRunner::new(String::from("127.0.0.1"), String::from("34254"), 20);

    server_node.start_sub_node("127.0.0.1".to_string(), "34254".to_string());;
}


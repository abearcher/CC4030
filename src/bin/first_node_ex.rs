mod node;

fn main() {
	println!("Starting first node!");
	let server_node = node::Node::new(String::from("127.0.0.1"), String::from("34254"));
	
	server_node.start_first_node();
}


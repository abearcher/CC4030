mod node;

fn main() {
	println!("Starting sub node!");
	let server_node = node::Node::new(String::from("1234"), String::from("127.0.0.1"));
	
	server_node.start_sub_node("127.0.0.1".to_string(), "34254".to_string());
	println!("end!");
}




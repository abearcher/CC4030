mod node;

fn main() {
	println!("Starting first node!");
	let server_node = node::Node{
		ip_list: Vec::new(),
		node_port: String::from("34254"),
		node_IP: String::from("127.0.0.1"),
	};
	
	server_node.start_first_node();
}


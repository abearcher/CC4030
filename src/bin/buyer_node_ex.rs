mod node;

fn main() {
	println!("Starting buyer first node!");
	let server_node = node::BuyerRunner::new(String::from("127.0.0.1"), String::from("1234"), 20);
	
	server_node.start_sub_node("127.0.0.1".to_string(), "34000".to_string());
	println!("end!");
}




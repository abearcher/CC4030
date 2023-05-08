use async_std::net::UdpSocket;

pub struct NodeServer {
	pub node_IP: String,
	pub node_port: String, 
	pub ID : Vec<u8>,
	pub ip_list: Vec<String>,
}

impl NodeServer {

	pub fn new(node_IP : String, node_port: String) -> NodeServer {
		let mut ret_node = NodeServer{
			node_IP: String::from(node_IP),
			node_port: String::from(node_port),
			ID : Vec::new(),
			ip_list: Vec::new(),
		};
		
		let IP = ret_node.node_IP.clone() + ret_node.node_port.as_str();
		ret_node.ID = ret_node.assign_ID(IP);
		println!("Assigning test ID to be");
		println!("{:?}", ret_node.ID);
		println!("{:?}", String::from_utf8_lossy(&ret_node.ID));
		//String::from_utf8_lossy(&buf)
		return ret_node;
	}



	/* ----- FUNCTIONS FOR SERVER ------- */
	fn rcv_ping(&self, rcv_ip : String, rcv_port : String){
		let ping_rcv_payload = json::object!(
			"sender_ip": self.node_IP.clone(),
			"sender_ip": self.node_port.clone()
		);
		
		let ping_reply = self.craft_command("PING_RECV".to_string(), ping_rcv_payload);
		
		//send ID
		self.send_command_to_node(rcv_ip, rcv_port, ping_reply);
		
	}
	
	


	
	async fn run_node_server(&self) -> std::io::Result<()> {
		let socket = UdpSocket::bind("127.0.0.1:34254").await?;
		println!("Starting server");
		
		let mut buf = vec![0u8; 1024];
		
		loop {
			let (n, peer) = socket.recv_from(&mut buf).await?;
			socket.send_to(&buf[..n], &peer).await?;
			println!("Received {} bytes from {}", n, peer);
			println!("Received message was {:?}", String::from_utf8_lossy(&buf));
			
			let received_string = &String::from_utf8_lossy(&buf[..n]);
			let parsed_command = json::parse(received_string).unwrap();
			
			println!("We have received {}", received_string);
			
			if parsed_command["command"] == "FIND_COMP" {
				println!("We have received FIND_COMP command");
			} else if parsed_command["command"] == "FIND_VALUE" {
				println!("We have received FIND_VALUE command");
			} else if parsed_command["command"] == "STORE" {
				println!("We have received STORE command");
			} else if parsed_command["command"] == "PING"{
				println!("We have received the PING command");
				self.rcv_ping(parsed_command["payload"]["IP"].to_string(), parsed_command["payload"]["PORT"].to_string());
			} else if parsed_command["command"] == "FIRST_JOIN"{
				println!("We have received the FIRST_JOIN command");
			} else {
				println!("Command not recognized!");
			}
				
				//println!("stuff inside code");
		}
	}
	


}

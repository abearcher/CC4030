use async_std::net::UdpSocket;
use async_std::task;
use sha1::{Sha1, Digest};
use std::fs::File;
use std::io::Write;

use crate::node::node_commands;

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
	async fn rcv_ping(&self, rcv_ip : String, rcv_port : String){
		println!("Sending ping response back!");
		let ping_rcv_payload = json::object!(
			"IP": self.node_IP.clone(),
			"PORT": self.node_port.clone()
		);
		
		let ping_reply = node_commands::craft_command("PING_RECV".to_string(), ping_rcv_payload);
		
		//send ID
		task::block_on(node_commands::send_command_to_node(rcv_ip, rcv_port, ping_reply));

		
	}
	
	
	fn assign_ID(&self, inputID: String) ->  Vec<u8> {
		//as per kademlia paper, uses sha-1
		//for now, we are usign the hash of the IP as the ID
		//using sha-1 per suggestion
		let mut hasher = Sha1::new();
		// process input message
		hasher.update(inputID);

		// acquire hash digest in the form of GenericArray,
		// which in this case is equivalent to [u8; 20]
		let result = hasher.finalize();
		return result.to_vec();
	}
	



	
	pub async fn run_node_server(&self) -> std::io::Result<()> {
	
		//let bind_to = self.node_IP.clone() + ":" + self.node_port.as_str();
		
		let bind_to = format!("{}:{}", self.node_IP, self.node_port);
		//let socket = UdpSocket::bind("127.0.0.1:34254").await?;
		println!("Starting server at {}", bind_to.to_string());
		let socket = UdpSocket::bind(bind_to).await?;

		
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
				
				task::block_on(self.rcv_ping(parsed_command["payload"]["IP"].to_string(), parsed_command["payload"]["PORT"].to_string()));

			} else if parsed_command["command"] == "FIRST_JOIN"{
				println!("We have received the FIRST_JOIN command");
			} else {
				println!("Command not recognized!");
			}
			
			self.write_to_file(received_string.to_string());
				
				//println!("stuff inside code");
		}
	}
	
	
	fn write_to_file(&self, output : String){
	
		let mut data_file = File::create("data.txt").expect("creation failed");
		data_file.write(output.as_bytes()).expect("write failed");
	}
	
	async fn communicate_with_client(socket: UdpSocket){
		println!("dsaf");
		
	    //receive_msg(socket);
	    //send_IPs(socket);
	}
	
	
	async fn send_IPs(socket: UdpSocket){

	}


}

use async_std::net::UdpSocket;
use async_std::{fs::File, io, prelude::*, task};
use futures::executor::block_on;
use std::thread;
use futures::join;
use sha1::{Sha1, Digest};
use json::JsonValue;


mod NodeServer;
mod cmd;
//mod cm;
use NodeCommand; // Use the NodeCommand struct
use cmd;


//use hex_literal::hex;

pub struct Node {
	pub node_IP: String,
	pub node_port: String, 
	pub ID : Vec<u8>,
	pub ip_list: Vec<String>,
}

impl Node {

	pub fn new(node_IP : String, node_port: String) -> Node {
		let mut ret_node = Node{
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

	pub fn start_first_node(&self) -> std::io::Result<()> {
		{

			println!("Starting first known node in a network");
			println!("test test testicles");
			
			//let server_node = NodeServer::NodeServer::new(self.node_IP.clone(), self.node_port.clone());
			//let handle = async_std::task::spawn(async move {
			//	self.run_node_server().await
			//});
			
			let run_server = NodeServer::NodeServer::new(self.node_IP.clone(), self.node_port.clone());
			task::spawn(async{
				//here we spawn a listener for incoming requests
				//this will respond to requests
				run_server.await;
			});
			//here we accept user input for different commands
			task::block_on(self.command_selection());
			
		} // the socket is closed here
			Ok(())
	}
	
	
	pub fn start_sub_node(&self, known_ip: String, known_port: String)  -> std::io::Result<()> {
		{
			println!("Connecting with known node!");
			
			let first_join_payload = json::object!(
				"sender_ip": self.node_IP.clone(),
				"sender_ip": self.node_port.clone()
			);
			
			//String::from("FIRST_JOIN")
			let first_join_cmd = cmd::craft_command("FIRST_JOIN".to_string(), first_join_payload);
			
			//self.craft_command();
			
			block_on(self.send_command_to_node(known_ip, known_port, first_join_cmd));
			println!("starting own server");
			//block_on(self.run_node_server());
			
			self.send_ping_cmd(known_ip, known_port);
			
			
			println!("end!")
		}
			Ok(())
	}
	

	/*fn craft_command(&self, command_in : String, payload_in: JsonValue) -> JsonValue {
		let mut out_command = json::object!{
		    "command": "",
		    "payload": {}
		};
		
		out_command["command"] = json::JsonValue::String(command_in);
		out_command["payload"] = payload_in;
		//json::JsonValue::String(payload);
		
		return out_command;
	}*/
	

	
	
	/* ----- FUNCTIONS FOR COMMUNICATION ------- 
	There should be 3 major commands here. PING, Store and GET*/
	
	fn full_ping_cmd(&self, ){
		//self.send_ping_cmd()
		self.wait_for_ping_reply()
	
	}
	
	fn send_ping_cmd(&self, receiverIP : String, receiverPort : String){
		//craft the ID as payload
		let ping_payload = json::object!(
			"sender_ip": self.node_IP.clone(),
			"sender_ip": self.node_port.clone()
		);
		
		let ping_command = cmd::craft_command("PING".to_string(), ping_payload);
		
		//send ID
		self.send_command_to_node(receiverIP, receiverPort, ping_command);
	}
	



	async fn communicate_with_client(socket: UdpSocket){
		println!("dsaf");
		
	    //receive_msg(socket);
	    //send_IPs(socket);
	}
	
	
	async fn send_IPs(socket: UdpSocket){

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
	

	async fn receive_msg(socket: UdpSocket) -> std::io::Result<()> {
		let mut buf = vec![0; 1024];
		let (n, peer) = socket.recv_from(&mut buf).await?;
		println!("Received {} bytes from {}", n, peer);
		
		//here we will try and parse our json
		let received_string = &String::from_utf8_lossy(&buf[..n]);
		let parsed_command = json::parse(received_string).unwrap();
		
		if parsed_command["command"] == "FIND_COMP" {
			println!("We have received {}", received_string);
		}
		else {
			println!("Command not recognized!");
		}

	    	Ok(())
	}
}

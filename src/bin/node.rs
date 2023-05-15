use async_std::net::UdpSocket;
use async_std::{fs::File, io, prelude::*, task};
use futures::executor::block_on;
use std::thread;
use futures::join;
use json::JsonValue;
use sha1::{Sha1, Digest};
use std::sync::{Mutex, Arc};


mod node_server;
mod node_commands;
mod node_client;
mod node_object;
mod seller;
mod buyer;
mod node_server_seller;

//mod cm;
use node_server::NodeServer; // Use the NodeCommand struct
//use cmd;


//use hex_literal::hex;

pub struct NodeRunner {
	node: Arc<Mutex<node_object::Node>>
	//pub node: node_object::Node
}

impl NodeRunner {

	pub fn new(node_IP : String, node_port: String, k : i32) -> NodeRunner {
		/*let mut ret_node = Node{
			node_IP: String::from(node_IP),
			node_port: String::from(node_port),
			//ID : Vec::new(),
			//ip_list: Vec::new(),
		};*/
		
		//let IP = ret_node.node_IP.clone() + ret_node.node_port.as_str();
		//ret_node.ID = ret_node.assign_ID(IP);
		//println!("Assigning test ID to be");
		//println!("{:?}", ret_node.ID);
		//println!("{:?}", String::from_utf8_lossy(&ret_node.ID));
		//String::from_utf8_lossy(&buf)

 		let node = Arc::new(Mutex::new(node_object::Node::new(node_IP, node_port, k)));
		NodeRunner {
			node: node,
		}

		//NodeRunner{
			//node: node_object::Node::new(node_IP, node_port)
		//}
	}

	pub fn start_first_node(&self) -> std::io::Result<()> {
		{

			println!("Starting first known node in a network");
			
			//let node = node_object::Node::new(node_IP, node_port)
			
			let node_clone = self.node.clone();
			let server_node = node_clone.clone();
			let client_node = node_clone.clone();
			
			//let server_node = NodeServer::NodeServer::new(self.node_IP.clone(), self.node_port.clone());
			//let handle = async_std::task::spawn(async move {
			//	self.run_node_server().await
			//});
			
			//let server_obj = node_server::NodeServer::new(self.node);
			//let run_server = server_obj.run_node_server();
			
			//task::block_on(server_obj.run_node_server());
			
			task::spawn(async move{
				//here we spawn a listener for incoming requests
				//this will respond to requests
				let mut server_obj = node_server::NodeServer::new(server_node);
				server_obj.run_node_server().await;
			});
			
			let client_obj = node_client::NodeClient::new(client_node);
			
			//here we accept user input for different commands
			task::block_on(client_obj.command_selection());
			
		} // the socket is closed here
			Ok(())
	}
	
	
	pub fn start_sub_node(&self, known_ip: String, known_port: String)  -> std::io::Result<()> {
		{
			println!("Connecting with known node!");
			
			let node_clone = self.node.clone();
			let server_node = node_clone.clone();
			let client_node = node_clone.clone();
			
			let this_node = self.node.lock().unwrap();
			
			let first_join_payload = json::object!(
				"sender_ip": this_node.node_IP.clone(),
				"sender_ip": this_node.node_port.clone()
			);
			
			drop(this_node);
			
			let node_clone = self.node.clone();
			
			//String::from("FIRST_JOIN")
			let first_join_cmd = node_commands::craft_command("FIRST_JOIN".to_string(), first_join_payload);
			
			//self.craft_command();
			
			block_on(node_commands::send_command_to_node(known_ip, known_port, first_join_cmd));
			println!("starting own server");
			//block_on(self.run_node_server());
			
			//self.send_ping_cmd(known_ip, known_port);
			
			//start server here
			//let server_obj = node_server::NodeServer::new(&self.node);
			
			task::spawn(async move{
				//here we spawn a listener for incoming requests
				//this will respond to requests
				let mut server_obj = node_server::NodeServer::new(server_node);
				server_obj.run_node_server().await;
			});
			
			let client_obj = node_client::NodeClient::new(client_node);
			
			//here we accept user input for different commands
			task::block_on(client_obj.command_selection());
			
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
	


	/*async fn receive_msg(socket: UdpSocket) -> std::io::Result<()> {
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
	}*/
}


pub struct SellerRunner {
	node: Arc<Mutex<node_object::Node>>
	//pub node: node_object::Node
}

impl SellerRunner {

	pub fn new(node_IP : String, node_port: String, k : i32) -> SellerRunner {

		let node = Arc::new(Mutex::new(node_object::Node::new(node_IP, node_port, k)));
		SellerRunner {
			node: node,
		}

	}

	pub fn start_first_node(&self) -> std::io::Result<()> {
		{

			println!("Starting first known node in a network");

			let node_clone = self.node.clone();
			let server_node = node_clone.clone();
			let client_node = node_clone.clone();


			task::spawn(async move{
				//here we spawn a listener for incoming requests
				//this will respond to requests
				let mut server_obj = node_server::NodeServer::new(server_node);
				server_obj.run_node_server().await;
			});

			let client_obj_in = node_client::NodeClient::new(client_node);
			let client_obj = seller::Seller::new(client_obj_in);

			//here we accept user input for different commands
			task::block_on(client_obj.command_selection());

		} // the socket is closed here
		Ok(())
	}


	pub fn start_sub_node(&self, known_ip: String, known_port: String)  -> std::io::Result<()> {
		{
			println!("Connecting with known node!");

			let node_clone = self.node.clone();
			let server_node = node_clone.clone();
			let client_node = node_clone.clone();

			let this_node = self.node.lock().unwrap();

			let first_join_payload = json::object!(
				"sender_ip": this_node.node_IP.clone(),
				"sender_ip": this_node.node_port.clone()
			);

			drop(this_node);

			let node_clone = self.node.clone();

			let first_join_cmd = node_commands::craft_command("FIRST_JOIN".to_string(), first_join_payload);

			block_on(node_commands::send_command_to_node(known_ip, known_port, first_join_cmd));
			println!("starting own server");


			task::spawn(async move{
				//here we spawn a listener for incoming requests
				//this will respond to requests
				let mut server_obj = node_server::NodeServer::new(server_node);
				server_obj.run_node_server().await;
			});

			let client_obj_in = node_client::NodeClient::new(client_node);
			let client_obj = seller::Seller::new(client_obj_in);

			//here we accept user input for different commands
			task::block_on(client_obj.command_selection());

			println!("end!")
		}
		Ok(())
	}
}
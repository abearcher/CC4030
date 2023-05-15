use async_std::net::UdpSocket;
use async_std::{fs::File, io, prelude::*, task};
use futures::executor::block_on;
use std::thread;
use futures::join;
use json::JsonValue;
use sha1::{Sha1, Digest};
use std::sync::{Mutex, Arc};

use crate::node::node_server;
use crate::node::node_commands;
use crate::node::node_client;
use crate::node::node_object;
use crate::node::seller;
use crate::node::node_server_seller; // Use the NodeCommand struct

/*
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
				let mut server_obj = node_server_seller::NodeServerSeller::new(server_node);
				server_obj.run_node_server().await;
			});
			
			let client_obj = seller::Seller::new(client_node);
			
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
				let mut server_obj = node_server_seller::NodeServerSeller::new(server_node);
				server_obj.run_node_server().await;
			});
			
			let client_obj = seller::Seller::new(client_node);
			
			//here we accept user input for different commands
			task::block_on(client_obj.command_selection());
			
			println!("end!")
		}
			Ok(())
	}
}*/

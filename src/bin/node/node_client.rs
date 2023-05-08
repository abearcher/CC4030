use async_std::{fs::File, io, prelude::*, task};

use crate::node::node_commands;

pub struct NodeClient {
	pub node_IP: String,
	pub node_port: String, 
}

impl NodeClient {
	
	pub fn new(node_IP : String, node_port: String) -> NodeClient {
		let mut ret_node = NodeClient{
			node_IP: String::from(node_IP),
			node_port: String::from(node_port),
		};

		return ret_node;
	}

	pub async fn command_selection(&self)-> io::Result<()> {
		let stdin = io::stdin();
		let mut line = String::new();
		loop{
			line = String::new();
			stdin.read_line(&mut line).await?;
			println!("Please select the following commands:\n1-PING Computer\n2- GET value\n3 - STORE value");
			println!("Selected {}", line);
			if line == "1"{
				println!("You have chosen to PING. Please select IP");
				line = String::new();
				stdin.read_line(&mut line).await?;
				let ip = line;
				line = String::new();
				println!("Please enter port");
				stdin.read_line(&mut line).await?;
				let port = line;
								
				//self.full_ping_cmd(ip, port);
				
			} else if line == "2"{
				//STORE();
			} else if line == "3" {
				//GET():
			} else {
				println!("Invalid Selection");
			}
		}
		Ok(())
	}


	fn send_ping_cmd(&self, receiverIP : String, receiverPort : String){
		//craft the ID as payload
		let ping_payload = json::object!(
			"sender_ip": self.node_IP.clone(),
			"sender_ip": self.node_port.clone()
		);
		
		let ping_command = node_commands::craft_command("PING".to_string(), ping_payload);
		
		//send ID
		node_commands::send_command_to_node(receiverIP, receiverPort, ping_command);
	}
}
	

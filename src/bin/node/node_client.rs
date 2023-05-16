use async_std::{fs::File, io, prelude::*, task};
use async_std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use hex;

use crate::node::node_commands;
use crate::node::node_object;


pub struct NodeClient {
	pub node_IP: String,
	pub node_port: String, 
	pub id : Vec<u8>,
	pub node : Arc<Mutex<node_object::Node>>
}

impl NodeClient {
	
	/*pub fn new(node_IP : String, node_port: String) -> NodeClient {
		let mut ret_node = NodeClient{
			node_IP: String::from(node_IP),
			node_port: String::from(node_port),
		};

		return ret_node;
	}*/
	
	
	pub fn new(node_in : Arc<Mutex<node_object::Node>>) -> NodeClient{
		let this_node = node_in.lock().unwrap();

		let ip_in = this_node.node_IP.clone();
		let port_in = this_node.node_port.clone();
		let id_in = this_node.ID.clone();
		drop(this_node);
	
		NodeClient{
			node_IP : ip_in,
			node_port : port_in,
			id : id_in,
			node : node_in,
		}	
	}

	pub async fn command_selection(&self)-> io::Result<()> {
		let stdin = io::stdin();
		
		
		let mut line = String::new();
		loop{
			let ip_of_sender = self.node_IP.clone();
			let port_of_sender = "4949".to_string();
			line = String::new();
			stdin.read_line(&mut line).await?;
			println!("Please select the following commands:\n1-PING Computer\n2 - STORE value\n3 - FIND VALUE\n4 - FIND COMPUTER/NODE");
			println!("Selected {}", line);
			if line.trim().to_string() == "1"{
				println!("You have chosen to PING. Please select IP");
				line = String::new();
				stdin.read_line(&mut line).await?;
				let ip = line.trim().to_string();
				line = String::new();
				println!("Please enter port");
				stdin.read_line(&mut line).await?;
				let port = line.trim().to_string();
				
				println!("t1, ip: {}, port {}", ip, port);
				task::block_on(self.full_ping_cmd(ip, port, ip_of_sender, port_of_sender));

				
			} else if line.trim().to_string() == "2"{
				//STORE(key, value);
				println!("You have chosen to STORE. Please select IP");
				line = String::new();
				stdin.read_line(&mut line).await?;
				let ip = line.trim().to_string();
				line = String::new();
				println!("Please enter port");
				stdin.read_line(&mut line).await?;
				let port = line.trim().to_string();
				line = String::new();
				println!("Please enter key to store");
				stdin.read_line(&mut line).await?;
				let key = line.trim().to_string();
				line = String::new();
				println!("Please enter value to store");
				stdin.read_line(&mut line).await?;
				let value = line.trim().to_string();
				
				println!("t1, ip: {}, port {}", ip, port);
				task::block_on(self.old_store(ip, port, key, value));

			} else if line.trim().to_string() == "3" {
				//FIND_VALUE(key):
								println!("You have chosen to STORE. Please select IP");
				line = String::new();
				stdin.read_line(&mut line).await?;
				let ip = line.trim().to_string();
				line = String::new();
				println!("Please enter port");
				stdin.read_line(&mut line).await?;
				let port = line.trim().to_string();
				line = String::new();
				println!("Please enter key to search for");
				stdin.read_line(&mut line).await?;
				let key = line.trim().to_string();
				
				println!("t1, ip: {}, port {}", ip, port);
				task::block_on(self.FIND_VALUE(ip, port, ip_of_sender, port_of_sender, key));

			} else if line.trim().to_string() == "4"{
				//FIND_COMP(id)
				line = String::new();
				stdin.read_line(&mut line).await?;
				let ip = line.trim().to_string();
				line = String::new();
				println!("Please enter port");
				stdin.read_line(&mut line).await?;
				let port = line.trim().to_string();
				//line = String::new();
				//println!("Please enter value to search for");
				//stdin.read_line(&mut line).await?;
				//let value = line.trim().to_string();
				
				println!("t1, ip: {}, port {}", ip, port);
				task::block_on(self.FIND_COMP(ip, port, ip_of_sender, port_of_sender, self.id.clone()));
			} else {
				println!("Invalid Selection");
			}
		}
		Ok(())
	}

	pub fn first_join(&self, send_to_ip: String, send_to_port : String) -> json::JsonValue {

		println!("Sending first join!");
		let ip_of_sender = self.node_IP.clone();
		let id = self.id.clone();
		let tmp_port_of_sender= "9999".to_string();

		let ret = task::block_on(self.FIND_COMP(send_to_ip, send_to_port, ip_of_sender, tmp_port_of_sender, id));

		return ret;

	}
	
	pub async fn FIND_VALUE(&self, send_to_ip : String, send_to_port : String, ip_of_sender : String, tmp_port_of_sender: String, key : String) -> json::JsonValue {
		let ip_of_sender_clone  = ip_of_sender.clone();
		let tmp_port_of_sender_clone = tmp_port_of_sender.clone();
		
		let payload = json::object!(
			"key": key,
		);
		
		let cmd = node_commands::craft_command("PING".to_string(), payload);
	
		let ret_str = task::block_on(node_commands::send_and_rcv_command(send_to_ip, send_to_port, cmd, ip_of_sender_clone, tmp_port_of_sender_clone));
		
		println!("What we got {}", ret_str);
		
		return ret_str;
	
	}
	
	pub async fn FIND_COMP(&self, send_to_ip : String, send_to_port : String, ip_of_sender : String, tmp_port_of_sender: String, id : Vec<u8>) -> json::JsonValue {
		let ip_of_sender_clone  = ip_of_sender.clone();
		let tmp_port_of_sender_clone = tmp_port_of_sender.clone();

		let payload = json::object!(
			"id": id,
		);

		let cmd = node_commands::craft_command("FIND_COMP".to_string(), payload);

		let ret_str = task::block_on(node_commands::send_and_rcv_command(send_to_ip, send_to_port, cmd, ip_of_sender_clone, tmp_port_of_sender_clone));

		println!("What we got {}", ret_str);

		return ret_str;
	}

	pub async fn store(&self, send_to_ip: String, send_to_port: String, key: String, value: node_object::StorageValue) {
		let value_payload = match value {
			node_object::StorageValue::Single(s) => json::JsonValue::String(s),
			node_object::StorageValue::Multiple(vec) => json::JsonValue::Array(
				vec.into_iter().map(json::JsonValue::String).collect(),
			),
		};

		let store_payload = json::object!(
			"key": key,
			"value": value_payload,
		);

		let store_cmd = node_commands::craft_command("STORE".to_string(), store_payload);

		task::block_on(node_commands::send_command_to_node(send_to_ip, send_to_port, store_cmd));
	}


	pub async fn old_store(&self, send_to_ip : String, send_to_port : String, key : String, value : String){
		let store_payload = json::object!(
			"key" : key,
			"value" : value,
		);

		let store_cmd = node_commands::craft_command("STORE".to_string(), store_payload);

		task::block_on(node_commands::send_command_to_node(send_to_ip, send_to_port, store_cmd));
	}


	 pub async fn public_store(send_to_ip : String, send_to_port : String, key : String, value : String){
		let store_payload = json::object!(
			"key" : key,
			"value" : value,
		);

		let store_cmd = node_commands::craft_command("STORE".to_string(), store_payload);

		task::block_on(node_commands::send_command_to_node(send_to_ip, send_to_port, store_cmd));
	}
	
	
	async fn full_ping_cmd(&self, send_to_ip : String, send_to_port : String, ip_of_sender : String, tmp_port_of_sender: String) -> json::JsonValue{
	
		let ip_of_sender_clone  = ip_of_sender.clone();
		let tmp_port_of_sender_clone = tmp_port_of_sender.clone();
		
		let ping_payload = json::object!(
			"IP": ip_of_sender,
			"PORT": tmp_port_of_sender
		);
		
		let ping_command = node_commands::craft_command("PING".to_string(), ping_payload);
	
		let ret_str = task::block_on(node_commands::send_and_rcv_command(send_to_ip, send_to_port, ping_command, ip_of_sender_clone, tmp_port_of_sender_clone));
		
		println!("What we got {}", ret_str);
		
		return ret_str;
	}
	
	/*async fn full_ping_cmd(&self, receiver_ip : String, receiver_port : String){
		println!("t3");
	
		println!("sending to {}:{}", receiver_ip, receiver_port);
		
		let sender_ip = self.node_IP.clone();
		let sender_port = "4444".to_string();
		
		let sender_ip_clone = sender_ip.clone();
		let sender_port_clone = sender_port.clone();
		
    		let ready_flag = Arc::new(Mutex::new(false));
    		let ready_flag_clone = Arc::clone(&ready_flag);
	
		let handle = task::spawn(
			async move{
					//here we spawn a listener for incoming requests
					//this will respond to requests
					NodeClient::tmp_srv(sender_ip, sender_port, ready_flag_clone).await;
		});
			
		while !*ready_flag.lock().unwrap() {
			task::yield_now().await;
		}
			
		task::block_on(self.send_ping_cmd(sender_ip_clone, sender_port_clone, receiver_ip, receiver_port));
		
		
		//handle.join().await.expect("Failed to join handle");
		handle.await;
		println!("after await")
		
   		//task::block_on(handle);
	
	}*/


	async fn send_ping_cmd(&self, node_IP : String, node_port : String, receiver_ip : String, receiver_port : String){
		//craft the ID as payload
		println!("Sending ping to {}:{}", receiver_ip, receiver_port);
		
		let ping_payload = json::object!(
			"IP": node_IP,
			"PORT": node_port
		);
		
		let ping_command = node_commands::craft_command("PING".to_string(), ping_payload);
		task::block_on(node_commands::send_command_to_node(receiver_ip, receiver_port, ping_command));
		println!("sent!")
	}
	
	/*pub async fn tmp_srv(node_IP : String, node_port : String,  ready_flag: Arc<Mutex<bool>>) -> std::io::Result<()> {
	
		let bind_to = format!("{}:{}", node_IP, node_port);
		//let socket = UdpSocket::bind("127.0.0.1:34254").await?;
		println!("Starting tmp server at {}", bind_to.to_string());
		let socket = UdpSocket::bind(bind_to).await?;
		println!("Starting server");
		
		let mut buf = vec![0u8; 1024];
		
		//let _ = ready_tx.send(());
		*ready_flag.lock().unwrap() = true;
		
		println!("yoyoyoyo");
		
		loop {
			let (n, peer) = socket.recv_from(&mut buf).await?;
			socket.send_to(&buf[..n], &peer).await?;
			println!("Received {} bytes from {}", n, peer);
			println!("Received message was {:?}", String::from_utf8_lossy(&buf));
			
			let received_string = &String::from_utf8_lossy(&buf[..n]);
			let parsed_command = json::parse(received_string).unwrap();
			
			println!("We have received {}", received_string);
			println!("received cmd is {}", parsed_command["command"].to_string());
			
			if parsed_command["command"].to_string() == "PING_RECV".to_string() {
				println!("We have received PING_RECV command");
			} else {
				println!("Command not recognized!");
			}
		}
	}*/

	pub fn parse_ip_port(&self, input: String) -> (String, String) {
		// Split the input string at the ':' character
		let parts: Vec<&str> = input.split(':').collect();

		// Ensure that the split produced exactly two parts
		let ip = parts[0].trim().to_string();
		let port = parts[1].trim().to_string();
		return (ip, port);
	}



	pub fn ret_storage(&self, key: &str) -> Option<node_object::StorageValue> {
		let this_node = self.node.lock().unwrap();
		let storage_map = this_node.storage.clone();
		drop(this_node);

		return storage_map.get(key).cloned()
	}

	pub fn print_storage(&self) {
		let this_node = self.node.lock().unwrap();
		this_node.print_storage();
	}
}
	

use async_std::{fs::File, io, prelude::*, task};
use async_std::net::UdpSocket;
use std::sync::{Arc, Mutex};

use crate::node::node_commands;
use crate::node::node_object;


pub struct NodeClient {
	pub node_IP: String,
	pub node_port: String, 
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
		drop(this_node);
	
		NodeClient{
			node_IP : ip_in,
			node_port : port_in,
			node : node_in,
		}	
	}

	pub async fn command_selection(&self)-> io::Result<()> {
		let stdin = io::stdin();
		let mut line = String::new();
		loop{
			line = String::new();
			stdin.read_line(&mut line).await?;
			println!("Please select the following commands:\n1-PING Computer\n2- GET value\n3 - STORE value");
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
				task::block_on(self.full_ping_cmd(ip, port, self.node_IP.clone(), "4949".to_string()));
				println!("t2");
				
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
	
	
	async fn full_ping_cmd(&self, send_to_ip : String, send_to_port : String, ip_of_sender : String, tmp_port_of_sender: String){
	
		let ip_of_sender_clone  = ip_of_sender.clone();
		let tmp_port_of_sender_clone = tmp_port_of_sender.clone();
		
		let ping_payload = json::object!(
			"IP": ip_of_sender,
			"PORT": tmp_port_of_sender
		);
		
		let ping_command = node_commands::craft_command("PING".to_string(), ping_payload);
	
		let test = task::block_on(node_commands::send_and_rcv_command(send_to_ip, send_to_port, ping_command, ip_of_sender_clone, tmp_port_of_sender_clone));
		
		println!("What we got {}", test);
	
	
	
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
	
}
	

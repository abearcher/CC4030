use async_std::net::UdpSocket;
use async_std::{fs::File, io, prelude::*, task};
use futures::executor::block_on;
use std::thread;
use futures::join;
use sha1::{Sha1, Digest};
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
			
			let run_server = Node::run_node_server();
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
			block_on(self.connect_with_node(known_ip, known_port, String::from("FIRST_JOIN")));
			println!("starting own server");
			//block_on(self.run_node_server());
			println!("end!")
		}
			Ok(())
	}
	
	async fn connect_with_node(&self, known_ip: String, known_port: String, select_command: String) -> std::io::Result<()> {
		
		//const THE_MERCHANT_OF_VENICE: &[u8] = b"
		//    If you prick us, do we not bleed?
		//    If you tickle us, do we not laugh?
		//    If you poison us, do we not die?
		//    And if you wrong us, shall we not revenge?
		//";
		
		const THE_MERCHANT_OF_VENICE: &[u8] = br#"{"command":"FIND_COMP"}"#;
		
		//let command = self.craft_command();

		let socket = UdpSocket::bind("127.0.0.1:0").await?;

		let addr = "127.0.0.1:34254";
		let sent = socket.send_to(THE_MERCHANT_OF_VENICE, &addr).await?;
		println!("Sent {} bytes to {}", sent, addr);
		Ok(())
	}
	
	async fn command_selection(&self)-> io::Result<()> {
		let stdin = io::stdin();
		let mut line = String::new();
		loop{
			line = String::new();
			stdin.read_line(&mut line).await?;
			println!("Please select the following commands:\n1-PING Computer\n2- GET value\n3 - STORE value");
			println!("Selected {}", line);
			if line == "1"{
				//self.send_ping_command();
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
	

	
	async fn run_node_server() -> std::io::Result<()> {
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
				//self.receive_ping(parsed_command["payload"]["IP"]);
			} else {
				println!("Command not recognized!");
			}
				
				//println!("stuff inside code");
		}
	}
	
	fn receive_ping(IP : String){
		println!("asdf");
		//send PING_RECEIVED to other message
		
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

use async_std::net::UdpSocket;
use async_std::{fs::File, io, prelude::*, task};
use futures::executor::block_on;
use std::thread;
use futures::join;


pub struct Node {
	pub ip_list: Vec<String>,
	pub node_port: String, 
	pub node_IP: String,
}

impl Node {
	pub fn start_first_node(&self) -> std::io::Result<()> {
		{
			//self.command_selection();
			//block_on(self.run_node_server());
			//self.run_node_server();
			
			
			println!("Running inter task");
			
			task::spawn(async { println!("Does it work here?"); });
			//let node_server = self.run_node_server();
		
			let f1 = Node::run_node_server();
			
			let test = Node::a_test_function();
			
			task::spawn(async{
				
				println!("right before a test function lol");
				//Node::a_test_function().await;
				f1.await;
			});
		
			//task::spawn(async { f1 });

			//task::block_on(self.inter_async());
			
			task::block_on(self.command_selection());
			

		} // the socket is closed here
			Ok(())
	}
	
	async fn a_test_function(){
		println!("Here I am running a test!!!!!");
	}
	
	async fn inter_async(&self){
		task::spawn(async { println!("wait a second this works!"); });
		
		block_on(self.command_selection());
	}
	
	pub fn start_sub_node(&self, known_ip: String, known_port: String)  -> std::io::Result<()> {
		{
			println!("connecting with known node!");
			block_on(self.connect_with_node(known_ip, known_port));
			println!("starting own server");
			//block_on(self.run_node_server());
			println!("end!")
		}
			Ok(())
	}
	
	async fn connect_with_node(&self, known_ip: String, known_port: String) -> std::io::Result<()> {
		
		//const THE_MERCHANT_OF_VENICE: &[u8] = b"
		//    If you prick us, do we not bleed?
		//    If you tickle us, do we not laugh?
		//    If you poison us, do we not die?
		//    And if you wrong us, shall we not revenge?
		//";
		
		const THE_MERCHANT_OF_VENICE: &[u8] = br#"{"command":"FIND_COMP"}"#;
		
		//const command = craft_command();

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
			println!("{}", line);
		}
		Ok(())
	}
	
	/*fn craft_command(&self, command : String, payload: String) -> json::object! {
		let command = json::object!{
		    "command": "",
		    "payload": {}
		};
		
		command["command"] = command;
		command["payload"] = json::JsonValue::String(payload);
		
		return command;
	}*/
	
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
			} else if parsed_command["command"] == "STORE"{
				println!("We have received the PING command");	
			} else {
				println!("Command not recognized!");
			}
				
				//println!("stuff inside code");
		}
	}
	
	async fn communicate_with_client(socket: UdpSocket){
		
		
	    //receive_msg(socket);
	    //send_IPs(socket);
	}
	
	
	async fn send_IPs(socket: UdpSocket){

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
		
	    //let mut buf = [0; 10];
	    //let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)
		//.expect("Didn't receive data");
	    //let filled_buf = &mut buf[..number_of_bytes];
	    	Ok(())
	}
	//here are the needed functions for Kademlia
	
	
	
}

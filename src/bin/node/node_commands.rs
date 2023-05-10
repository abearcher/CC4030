use json::JsonValue;
use async_std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use async_std::task;
use std::result::Result;


pub fn craft_command(command_in : String, payload: JsonValue) -> JsonValue {
	let mut command = json::object!{
	    "command": "",
	    "payload": {}
	};
	
	command["command"] = json::JsonValue::String(command_in);
	command["payload"] = payload;
	
	return command;
}


pub async fn send_command_to_node(send_to_ip: String, send_to_port: String, crafted_cmd: JsonValue) -> std::io::Result<()> {
	
	//const THE_MERCHANT_OF_VENICE: &[u8] = b"
	//    If you prick us, do we not bleed?
	//    If you tickle us, do we not laugh?
	//    If you poison us, do we not die?
	//    And if you wrong us, shall we not revenge?
	//";
	
	let addr = format!("{}:{}", send_to_ip, send_to_port);
	
	println!("----------------------fart");
	println!("Sending to --{}--", addr);
	
	let THE_MERCHANT_OF_VENICE: Vec<u8> = crafted_cmd.to_string().into_bytes();
	//br#"{"command":"FIND_COMP"}"#;

	let socket = UdpSocket::bind("127.0.0.1:0").await?;

	//let addr = "127.0.0.1:34254";
	let sent = socket.send_to(&THE_MERCHANT_OF_VENICE, &addr).await?;
	println!("Sent {} bytes to {}", sent, addr);
	Ok(())
}

pub async fn send_and_rcv_command(send_to_ip: String, send_to_port: String, crafted_cmd: JsonValue, ip_of_sender : String, ip_of_port : String) -> json::JsonValue{

		println!("sending to {}:{}", send_to_ip, send_to_port);
			
    		let ready_flag = Arc::new(Mutex::new(false));
    		let ready_flag_clone = Arc::clone(&ready_flag);
    		let parsed_command = Arc::new(Mutex::new(None));
    		let parsed_command_clone = Arc::clone(&parsed_command);
	
		let handle = task::spawn(
			async move{
					//here we spawn a listener for incoming requests
					//this will respond to requests
					tmp_srv(ip_of_sender, ip_of_port, ready_flag_clone, parsed_command_clone).await;
		});
			
		while !*ready_flag.lock().unwrap() {
			task::yield_now().await;
		}
			
		task::block_on(send_command_to_node(send_to_ip, send_to_port, crafted_cmd));

		handle.await;
		
		let ret_cmd = parsed_command.lock().unwrap().take().unwrap();
		
		return ret_cmd;
}


pub async fn tmp_srv(ip_of_sender : String, ip_of_port : String, ready_flag: Arc<Mutex<bool>>, parsed_command: Arc<Mutex<Option<JsonValue>>>) -> std::io::Result<()> {
	
		let bind_to = format!("{}:{}", ip_of_sender, ip_of_port);
		println!("Starting tmp server at {}", bind_to.to_string());
		let socket = UdpSocket::bind(bind_to).await?;
		println!("Starting server");
		
		let mut buf = vec![0u8; 1024];
		
		//let _ = ready_tx.send(());
		*ready_flag.lock().unwrap() = true;
		
		println!("yoyoyoyo");
		

		let (n, peer) = socket.recv_from(&mut buf).await?;
		socket.send_to(&buf[..n], &peer).await?;
		println!("Received {} bytes from {}", n, peer);
		println!("Received message was {:?}", String::from_utf8_lossy(&buf));
		
		let received_string = &String::from_utf8_lossy(&buf[..n]);
		let parsed_command_value = json::parse(received_string).unwrap();
		
		*parsed_command.lock().unwrap() = Some(parsed_command_value);
		
		Ok(())

	}







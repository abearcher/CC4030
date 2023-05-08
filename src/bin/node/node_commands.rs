use json::JsonValue;
use async_std::net::UdpSocket;

pub fn craft_command(command_in : String, payload: JsonValue) -> JsonValue {
	let mut command = json::object!{
	    "command": "",
	    "payload": {}
	};
	
	command["command"] = json::JsonValue::String(command_in);
	command["payload"] = payload;
	
	return command;
}


pub async fn send_command_to_node(known_ip: String, known_port: String, crafted_cmd: JsonValue) -> std::io::Result<()> {
	
	//const THE_MERCHANT_OF_VENICE: &[u8] = b"
	//    If you prick us, do we not bleed?
	//    If you tickle us, do we not laugh?
	//    If you poison us, do we not die?
	//    And if you wrong us, shall we not revenge?
	//";
	
	let THE_MERCHANT_OF_VENICE: Vec<u8> = crafted_cmd.to_string().into_bytes();
	//br#"{"command":"FIND_COMP"}"#;

	let socket = UdpSocket::bind("127.0.0.1:0").await?;

	let addr = "127.0.0.1:34254";
	let sent = socket.send_to(&THE_MERCHANT_OF_VENICE, &addr).await?;
	println!("Sent {} bytes to {}", sent, addr);
	Ok(())
}







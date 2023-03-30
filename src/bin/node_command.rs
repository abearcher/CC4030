pub struct NodeCommand {

}

impl NodeCommand {
	fn craft_command(&self, command : String, payload: String) -> json::object! {
		let command = json::object!{
		    "command": "",
		    "payload": {}
		};
		
		command["command"] = command;
		command["payload"] = json::JsonValue::String(payload);
		
		return command;
	}	
}

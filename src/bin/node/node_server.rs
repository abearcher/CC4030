use async_std::net::UdpSocket;
use async_std::task;
use sha1::{Sha1, Digest};
use std::fs::File;
use std::io::Write;
use std::sync::{Mutex, Arc};
use std::any::Any;
//use bytes::{BigEndian, ByteOrder};

use crate::node::node_commands;
use crate::node::node_object;

pub struct NodeServer  {
	pub node_IP: String,
	pub node_port: String, 
	//pub ID : Vec<u8>,
	//pub ip_list: Vec<String>,
	pub node : Arc<Mutex<node_object::Node>>
}

impl  NodeServer  {

	/*pub fn new(node_IP : String, node_port: String) -> NodeServer {
		let mut ret_node = NodeServer{
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
	}*/

	pub fn new(node_in : Arc<Mutex<node_object::Node>>) -> NodeServer {
		//let node = node_in.lock().unwrap();

		let this_node = node_in.lock().unwrap();

		let ip_in = this_node.node_IP.clone();
		let port_in = this_node.node_port.clone();
		drop(this_node);
	
		NodeServer{
			node_IP : ip_in,
			node_port : port_in,
			node : node_in
		}	
	}


	/* ----- FUNCTIONS FOR SERVER ------- */
	async fn rcv_ping(&self, rcv_ip : String, rcv_port : String){
		println!("Sending ping response back!");
		let ping_rcv_payload = json::object!(
			"IP": self.node_IP.clone(),
			"PORT": self.node_port.clone()
		);
		
		let ping_reply = node_commands::craft_command("PING_RECV".to_string(), ping_rcv_payload);
		
		//send ID
		task::block_on(node_commands::send_command_to_node(rcv_ip, rcv_port, ping_reply));

		
	}
	
	
	async fn rcv_find_value(&self, rcv_ip : String, rcv_port : String, value_or_list : String, ret_value : String){
		println!("Sending ping response back!");
		let find_value_payload = json::object!(
			"value_or_list": value_or_list,
			"value": ret_value
		);
		
		let reply = node_commands::craft_command("FIND_VALUE_RECV".to_string(), find_value_payload);
		
		//send ID
		task::block_on(node_commands::send_command_to_node(rcv_ip, rcv_port, reply));
		
	}
	
	async fn rcv_find_comp(&self, rcv_ip : String, rcv_port : String, comp_str : String){
	
		println!("Sending ping response back!");
		let find_value_payload = json::object!(
			"comp_list": comp_str
		);
		
		let reply = node_commands::craft_command("FIND_COMP_RCV".to_string(), find_value_payload);
		
		//send ID
		task::block_on(node_commands::send_command_to_node(rcv_ip, rcv_port, reply));
	
	
	}


	pub async fn run_node_server(&self) -> std::io::Result<()> {
	
		//let bind_to = self.node_IP.clone() + ":" + self.node_port.as_str();
		
		let bind_to = format!("{}:{}", self.node_IP, self.node_port);
		//let socket = UdpSocket::bind("127.0.0.1:34254").await?;
		println!("Starting server at {}", bind_to.to_string());
		let socket = UdpSocket::bind(bind_to).await?;

		
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
				let id = parsed_command["payload"]["ID"].to_string().as_bytes().to_vec();
				let ret_find_comp = self.FIND_COMP(id);
				task::block_on(self.rcv_find_comp(parsed_command["payload"]["IP"].to_string(), parsed_command["payload"]["PORT"].to_string(), ret_find_comp));
			} else if parsed_command["command"] == "FIND_VALUE" {
				println!("We have received FIND_VALUE command");
				let key = parsed_command["payload"]["key"].to_string();
				let (ret_val, value_or_list )= self.FIND_VALUE(key);
				task::block_on(self.rcv_find_value(parsed_command["payload"]["IP"].to_string(), parsed_command["payload"]["PORT"].to_string(), ret_val, value_or_list));
			} else if parsed_command["command"] == "STORE" {
				println!("We have received STORE command");
				let key = parsed_command["payload"]["key"].to_string();
				let value = parsed_command["payload"]["value"].to_string();
				self.STORE(key, value);
				
				let this_node = self.node.lock().unwrap();
    				//this_node.storage.insert(key, value);
				for (key, value) in &this_node.storage {
					println!("{}: {}", key, value);
				}
				drop(this_node);
				
			} else if parsed_command["command"] == "PING"{
				println!("We have received the PING command");
				
				task::block_on(self.rcv_ping(parsed_command["payload"]["IP"].to_string(), parsed_command["payload"]["PORT"].to_string()));

			} else if parsed_command["command"] == "FIRST_JOIN"{
				println!("We have received the FIRST_JOIN command");
			} else {
				println!("Command not recognized!");
			}
			
			//self.write_to_file(received_string.to_string());
				
				//println!("stuff inside code");
		}
	}
	
	fn FIND_COMP(&self, id : Vec<u8>) -> Vec<node_object::RoutingTablePair> {
	
		let mut min = node_object::RoutingTablePair::new("".to_string(),Vec::new());
		let mut min_index = 0;
	
		//let list_of_closest_comps = Vec::new();
		let mut list_of_closest_comps: Vec<node_object::RoutingTablePair> = Vec::new();
	
		//let this_node = node_in.lock().unwrap();
		let this_node = self.node.lock().unwrap();
		let k = this_node.k.clone();
		let mut list = this_node.routing_table.clone();
		drop(this_node);
		
		for i in 0..k-1{
			(min, min_index) = self.find_min_dist(id.clone(), list.clone());
			list_of_closest_comps.push(min);
			list.remove(min_index.try_into().unwrap());
		}
		
		return list_of_closest_comps;
	}


	fn find_min_dist(&self, id : Vec<u8>, list: Vec<node_object::RoutingTablePair>) -> (node_object::RoutingTablePair, i32){
		//let this_node = self.node.lock().unwrap();
		//let len_of_id_list = this_node.ip_list.clone().len();
		//let list = this_node.ip_list.clone();
		//drop(this_node);
		let len_of_id_list = list.len();
		let mut counting_dist = self.ret_max_value();
		//std::i32::MAX;
		let mut cur_id = id.clone();
		let mut counting_dist_index = 0;
		let mut ret_rout_table_pair = node_object::RoutingTablePair::new("".to_string(), Vec::new());
		
		for i in 0..len_of_id_list{
			//dist = cur_id ^ list[i].id;
			let mut dist = Vec::new();
			
			for (a, b) in cur_id.iter().zip(list[i].id.clone()) {
			    dist.push(a ^ b);
			}
			
			let dist_val = u128::from_be_bytes(dist[..].try_into().unwrap());
			
			if  self.is_vec1_smaller_than_vec2(dist.clone(), counting_dist.clone()){
			//dist < counting_dist
				counting_dist = dist.clone();
				cur_id = list[i].clone().id;
				counting_dist_index = i;
				
				ret_rout_table_pair.id = list[i].id.clone();
				ret_rout_table_pair.ip = list[i].ip.clone();
			}
		}
		
		return (ret_rout_table_pair, counting_dist_index.try_into().unwrap());	
	}
	
	fn is_vec1_smaller_than_vec2(&self, vec1 : Vec<u8>, vec2: Vec<u8>) -> bool {
		let mut i = 0;
		while i < vec1.len() && i < vec2.len() {
		    if vec1[i] < vec2[i] {
			return true;
		    } else if vec1[i] > vec2[i] {
			return false;
		    }
		    i += 1;
		}
		return false;
	}
	
	fn ret_max_value(&self) -> Vec<u8>{
		let mut v = Vec::new();
		let mut max_byte: u8 = 0xFF; // initialize to maximum possible value
		for i in 0..20{
			v.push(max_byte);
		}
		return v;
	}
	
	fn STORE(&self, key : String, value : String){
		let mut this_node = self.node.lock().unwrap();
    		this_node.storage.insert(key, value);
	}
	
	/*fn format_find_cmp_as_str(list : Vec<node_object::RoutingTablePair>) -> String{
		//Vec<node_object::RoutingTablePair>
		
		let mut ret_str = "".to_string();
		
		for i in 0..list.len(){
			if i > 0 {
				ret_str = ret_str + ",".to_string();
			}
		
			let tmp_str = format!("({},{})", list[i].ip, list[i].id).to_string();
			ret_str = ret_str + tmp_str;
			//format!("{}{}", ret_str.clone(), tmp_str.clone());
		}
		
		//ret_str = ret_str + "";
		
		return ret_str;
	
	}*/
	
	fn format_find_cmp_as_str(&self, list: Vec<node_object::RoutingTablePair>) -> String {
		let mut ret_str = String::new();

		for (i, pair) in list.iter().enumerate() {
			if i > 0 {
			    ret_str.push(',');
			}

			let id_str = String::from_utf8_lossy(&pair.id).to_string();
			let pair_str = format!("({}, {})", pair.ip, id_str);
			ret_str.push_str(&pair_str);
		}

		ret_str
	}

	
	fn FIND_VALUE(&self, key : String) -> (String, String) {
	
		let this_node = self.node.lock().unwrap();
		let storage_map = this_node.storage.clone();
		let list = this_node.routing_table.clone();
		drop(this_node);
		
		let ret_value = storage_map.get(&key);
	
		if let Some(value) = ret_value {
			 return (value.to_owned(), "value".to_string());
		} else {
			let ret_comp = self.FIND_COMP(key.as_bytes().to_vec());
			return (self.format_find_cmp_as_str(ret_comp), "value".to_string());
		}
		
	}

	
	
	fn write_to_file(&self, output : String){
	
		let mut data_file = File::create("data.txt").expect("creation failed");
		data_file.write(output.as_bytes()).expect("write failed");
	}
	
	async fn communicate_with_client(socket: UdpSocket){
		println!("dsaf");
		
	    //receive_msg(socket);
	    //send_IPs(socket);
	}
	
	
	async fn send_IPs(socket: UdpSocket){

	}


}

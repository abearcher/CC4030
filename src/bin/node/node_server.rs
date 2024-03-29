use async_std::net::UdpSocket;
use async_std::task;
use sha1::{Sha1, Digest};
use std::fs::File;
use std::io::Write;
use std::sync::{Mutex, Arc};
use std::any::Any;
use json::{object, JsonValue};
use std::option::Option;
use serde_json;
use crate::node;

use crate::node::buyer::models;

use crate::node::{node_client, node_commands};
use crate::node::node_object;
use crate::node::node_object::RoutingTablePair;

pub struct NodeServer  {
	pub node_IP: String,
	pub node_port: String,
	pub node : Arc<Mutex<node_object::Node>>
}

impl  NodeServer  {

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
	
	async fn rcv_find_comp(&self, rcv_ip : String, rcv_port : String, routing_table_vector : Vec<RoutingTablePair>){
		//TODO: return k closest computers based off of ID

		let mut comp_str = Vec::new();
		for pair in routing_table_vector {
			let obj = object! {
				"ip": pair.ip,
				"port": pair.port,
				"id": pair.id};

			comp_str.push(obj);
		}

		println!("Sending ping response back!");
		let find_value_payload = json::object!(
			"comp_list": comp_str
		);
		
		let reply = node_commands::craft_command("FIND_COMP_RCV".to_string(), find_value_payload);
		
		//send ID
		task::block_on(node_commands::send_command_to_node(rcv_ip, rcv_port, reply));
	
	
	}

	pub async fn run_node_server(&self) -> std::io::Result<()> {
		
		let bind_to = format!("{}:{}", self.node_IP, self.node_port);
		//let socket = UdpSocket::bind("127.0.0.1:34254").await?;
		println!("Starting server at {}", bind_to.to_string());
		let socket = UdpSocket::bind(bind_to).await?;

		
		let mut buf = vec![0u8; 5000];
		
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
				let (ret_val, value_or_list )= self.find_value(key);
				task::block_on(self.rcv_find_value(parsed_command["payload"]["IP"].to_string(), parsed_command["payload"]["PORT"].to_string(), ret_val, value_or_list));
			} else if parsed_command["command"] == "STORE" {
				println!("We have received STORE command");

				self.store_command(parsed_command.clone());

			} else if parsed_command["command"] == "PING"{
				println!("We have received the PING command");

				task::block_on(self.rcv_ping(parsed_command["payload"]["IP"].to_string(), parsed_command["payload"]["PORT"].to_string()));
			} else {
				println!("Command not recognized!");
			}
		}
	}
	
	fn FIND_COMP(&self, id : Vec<u8>) -> Vec<node_object::RoutingTablePair> {

		//TODO: add check that if the IP is new, and closest in the k bucket closes nodes, we add that to the routing table
		//and remove the k least closes if needed
	
		let mut min = node_object::RoutingTablePair::new("".to_string(), "".to_string(),Vec::new());
		let mut min_index = 0;
	
		//let list_of_closest_comps = Vec::new();
		let mut list_of_closest_comps: Vec<node_object::RoutingTablePair> = Vec::new();
	
		//let this_node = node_in.lock().unwrap();
		let this_node = self.node.lock().unwrap();
		let k = this_node.k.clone();
		let mut list = this_node.routing_table.clone();
		drop(this_node);
		
		for i in 0..list.len(){
			//println!("iteration: {}", i);
			//println!("list len: {}", list.len());
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
		let mut ret_rout_table_pair = node_object::RoutingTablePair::new("".to_string(), "".to_string(),Vec::new());
		
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
	
	/*fn store(&self, key : String, value : String){
		let mut this_node = self.node.lock().unwrap();
		this_node.storage.insert(key, value);
	}*/


	fn store_command(&self, parsed_command: JsonValue) {
		let key = parsed_command["payload"]["key"].as_str().unwrap().to_string();

		let value_json = parsed_command["payload"]["value"].clone();

		println!("The json key: {}, the json value {}", key, value_json);
		println!("is array {}, is string {}", value_json.is_array(), value_json.is_string());

		if key.clone() == "active_auction" && value_json.clone() == "false" {
			self.end_auction();
		}

		if value_json.is_string() && key.clone() != "bid" {
			let value = node_object::StorageValue::Single(value_json.to_string());
			self.store(key, value);
		} else if value_json.is_array() {
			let value = node_object::StorageValue::Multiple(self.json_list_to_str_vec(value_json));
			self.store(key, value);
		} else {
			panic!("Invalid value type")
		}


	}

	fn json_list_to_str_vec(&self, jval : JsonValue) -> Vec<String> {
		let mut ret_vec = Vec::new();
		for i in 0 .. jval.len(){
			ret_vec.push(jval[i].clone().to_string());
		}
		return ret_vec;
	}



	fn end_auction(&self){
		println!("THE AUCTION IS OVER!!!!!");
		//grab largest bid
		let largest_json_bid = self.grab_largest_bid();
		//save largest bid

		//grab value from hashmap
		let blockchain_get= self.ret_storage("blockchain");

		if let Some(node_object::StorageValue::Single(block_val)) = blockchain_get {
			//deserialize the blockchain
			let mut blockchain = serde_json::from_str(&*block_val).expect("Deserialization failed");

			//add new block
			models::blockchain::Blockchain::add_block(&mut blockchain, largest_json_bid);

			//re-serialize
			let re_json = serde_json::to_string(&blockchain).expect("Serialization failed");

			//replace old blockchain with new
			let send_info = node_object::StorageValue::Single(re_json.clone());

			println!("the final info for the auction is: {}", re_json.clone());

			task::block_on(node::node_client::NodeClient::public_store(self.node_IP.clone(), self.node_port.clone(), "blockchain".to_string(), send_info));

			//let mut this_node = self.node.lock().unwrap();
			//this_node.storage["blockchain"] = re_json;

		}


	}

	fn grab_largest_bid(&self) -> (String) {
		let bids_get= self.ret_storage("bids");
		let mut ret_json = "".to_string();

		if let Some(node_object::StorageValue::Multiple(bids)) = bids_get {
			let mut ret_bid : i32 = bids[0].parse().unwrap();
			let mut ret_index : i32 = 0;
			for i in 1..bids.len() {
				//println!("{}", cur_bid);
				let json_bid = json::parse(&*bids[i].clone()).unwrap();
				let cur_bid: i32 = json::stringify(json_bid["bid"].clone()).parse().unwrap();

				if cur_bid > ret_bid {
					ret_bid = cur_bid;
					ret_index = i as i32;
					ret_json = json::stringify(json_bid.clone());
				}

				//println!("{}", json["bid"]);
			}
			println!("largest bid for auction: {}", ret_bid);
			println!("json return value: {}", ret_json);
			return ret_json;
		}

		return ret_json;
	}

	fn store(&self, key: String, new_value: node_object::StorageValue) {
		let mut this_node = self.node.lock().unwrap();
		//this_node.storage.insert(key, value);

		match this_node.storage.get_mut(&key) {
			Some(node_object::StorageValue::Single(_)) => {
				this_node.storage.insert(key, new_value);
			},
			Some(node_object::StorageValue::Multiple(values)) => {
				match new_value {
					node_object::StorageValue::Single(value) => {
						values.push(value.clone());

						//if our key is storing to bids AND condition is_seller
						if(key.to_string() == "bids" && self.is_seller()){
							self.publish_bids(key.clone(), value.clone());
						}
					},
					node_object::StorageValue::Multiple(new_values) => {
						values.extend(new_values);
					},
				}
			},
			None => {
				this_node.storage.insert(key, new_value);
			},
		}


	}

	fn is_seller(&self) -> bool {
		let this_node = self.node.lock().unwrap();
		let storage_map = this_node.storage.clone();
		drop(this_node);

		if let Some(value) = storage_map.get("is_seller") {
			match value {
				node_object::StorageValue::Single(s) => return s.to_string() == "true",
				node_object::StorageValue::Multiple(v) => return v.contains(&"true".to_string()),
			}
		}
		false
	}


	fn publish_bids(&self, key : String, value : String) {

		//when a seller receives bids, it publishes them to its subscribers
		//uses the STORE method

		//for every seller IP
		let list_of_ips = self.ret_storage("buyer_ips");

		// Check if the storage value is of type StorageValue::Multiple
		if let Some(node_object::StorageValue::Multiple(values)) = list_of_ips {
			for ip_port in values {
				// Store the value to the client
				// Key: ID of the seller
				// Value: New bid info
				let (ip, port) = self.parse_ip_port(ip_port);
				node_client::NodeClient::public_store(ip, port, key.clone(), node::node_object::StorageValue::Single(value.clone()));
			}
		} else {
			println!("bids unable to be purchased");
		}
	}

	fn parse_ip_port(&self, input: String) -> (String, String) {
		// Split the input string at the ':' character
		let parts: Vec<&str> = input.split(':').collect();

		// Ensure that the split produced exactly two parts
		let ip = parts[0].trim().to_string();
		let port = parts[1].trim().to_string();
		return (ip, port);
	}



	fn ret_storage(&self, key: &str) -> Option<node_object::StorageValue> {
		let this_node = self.node.lock().unwrap();
		let storage_map = this_node.storage.clone();
		drop(this_node);

		return storage_map.get(key).cloned()
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

	
	/*fn find_value(&self, key : String) -> (String, String) {
	
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
		
	}*/

	fn find_value(&self, key: String) -> (String, String) {
		let this_node = self.node.lock().unwrap();
		let storage_map = this_node.storage.clone();
		let list = this_node.routing_table.clone();
		drop(this_node);

		if let Some(storage_value) = storage_map.get(&key) {
			match storage_value {
				node_object::StorageValue::Single(s) => (s.to_owned(), "value".to_string()),
				node_object::StorageValue::Multiple(l) => (l.join(","), "list".to_string()),
			}
		} else {
			let ret_comp = self.FIND_COMP(key.as_bytes().to_vec());
			(self.format_find_cmp_as_str(ret_comp), "value".to_string())
		}
	}
}

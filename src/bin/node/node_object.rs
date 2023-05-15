use std::sync::Mutex;
use sha1::{Sha1, Digest};
use std::collections::HashMap;

#[derive(Clone)]
pub struct RoutingTablePair{
	pub ip: String,
	pub port: String,
   	pub id: Vec<u8>,
}

#[derive(Clone)]
pub enum StorageValue {
	Single(String),
	Multiple(Vec<String>),
}

pub struct Node {
	pub node_IP: String,
	pub node_port: String, 
	pub ID : Vec<u8>,
	//pub ip_list: Mutex<Vec<String>>
	pub routing_table: Vec<RoutingTablePair>,
	pub k : i32,
	pub storage: HashMap<String, StorageValue>,
	//pub ip_list: Vec<String>,
}


impl RoutingTablePair{

	pub fn new(ip_in : String, port_in : String, id_in : Vec<u8>) -> RoutingTablePair{
		let mut ret = RoutingTablePair{
			ip: ip_in,
			port: port_in,
   			id: id_in,
   		};
   		
   		return ret;
	}

}

impl Node {

	pub fn new(node_IP : String, node_port: String, k_buckets_in : i32) -> Node {
	
		
		let IP_port = node_IP.clone() + node_port.as_str();
		let ret_ID = Node::assign_ID(IP_port);
		//println!("Assigning test ID to be");
		//println!("{:?}", ret_node.ID);
		//println!("{:?}", String::from_utf8_lossy(&ret_node.ID));
		//String::from_utf8_lossy(&buf)
		
		let ret_node = Node{
			node_IP: String::from(node_IP),
			node_port: String::from(node_port),
			ID : ret_ID,
			//ip_list: Mutex::new(Vec::new()),
			routing_table: Vec::new(),
			k : k_buckets_in,
			storage : HashMap::new(),
		};
		
		ret_node
	}
	
	
	pub fn assign_ID(inputID: String) ->  Vec<u8> {
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

	pub fn print_storage(&self) {
		println!("printing storage!");
		for (key, value) in &self.storage {
			match value {
				StorageValue::Single(s) => println!("Key: {}, Value: {}", key, s),
				StorageValue::Multiple(vec) => {
					let values_str = vec.join(", ");
					println!("Key: {}, Value: [{}]", key, values_str);
				}
			}
		}
	}

	/*pub fn add_ip_to_list(&self, ip: String) {
		let mut ip_list = self.ip_list.lock().unwrap();
		ip_list.push(ip);
	}*/

}

//import nodeClient
use crate::node::node_server;
use async_std::task;
use crate::node;
use crate::node::node_client;
use std::sync::{Mutex, Arc};
/*
pub struct NodeServerSeller{
	node_server: node_server::NodeServer,

	//list of subscribers?
}

impl NodeServerSeller {

	pub fn new(node_mutex : Arc<Mutex<node::node_object::Node>>) -> NodeServerSeller {

		let node_server_new = node::node_server::NodeServer::new(node_mutex);
		return NodeServerSeller{
			node_server : node_server_new,

		};
	}

	fn STORE(&self, key : String, value : String){
		let mut this_node = self.node_client.node.lock().unwrap();
    		this_node.storage.insert(key, value);
    		
    		self.publish_bids(key.clone(), value.clone());
	}
	
	fn publish_bids(&self, key : String, value : String) {

		//when a seller receives bids, it publishes them to its subscribers
		//uses the STORE method

		//for every seller IP
		let list_of_ips = self.node_client.ret_storage("buyer_ips");

		for (send_to_ip, send_to_port) in &list_of_ips {
			//store value to client
			//key: id of seller
			//value: new bid info
			self.node_client.store(send_to_ip, send_to_port, key, value);
		}
	}
	
	
	fn ret_storage_subkey_values(&self, subkey: String){
		let mut this_node = self.node.lock().unwrap();
    		//this_node.storage.insert(key, value);
	
		self.node_client.ret_storage_subkey_values(subkey)
	}
}
*/

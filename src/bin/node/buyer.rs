//import nodeClient
use crate::node::node_client;
use crate::node::node_commands;
use crate::node::node_object::StorageValue;
use async_std::{fs::File, io, prelude::*, task};
use json::JsonValue;


pub struct Buyer{
	node_client: node_client::NodeClient,
	//list of subscribers?
}


//in buyer's hashmap
//subscribed_to : Vector of Strings (<ip>:<port>)
//is_buyer

impl Buyer {

	pub fn new(node_client_in : node_client::NodeClient) -> Buyer{

		return Buyer{
			node_client: node_client_in,
		}

	}

	pub async fn command_selection(&self)-> io::Result<()> {
		let stdin = io::stdin();


		let mut line = String::new();
		loop{
			//let ip_of_sender = self.node_IP.clone();
			//let port_of_sender = "4949".to_string();
			println!("\nPlease select the following commands:\n1 - List All Sellers\n2 - subscribe to seller\n3 - send bid to seller");
			println!("Selected {}", line);
			line = String::new();
			stdin.read_line(&mut line).await?;

			if line.trim().to_string() == "1"{
				println!("You have chosen to list all sellers");
				self.ask_for_seller_list();
				println!("sellers have been listed!");
			} else if line.trim().to_string() == "2"{
				//STORE(key, value);
				println!("You chose to subscribe to seller");
				println!("Enter IP of seller");
				line = String::new();
				stdin.read_line(&mut line).await?;
				let buyer_ip = line.trim().to_string();
				println!("Enter port of seller");
				line = String::new();
				stdin.read_line(&mut line).await?;
				let buyer_port = line.trim().to_string();
				//self.subscribe_to_buyer(buyer_ip, buyer_port)

			} else if line.trim().to_string() == "3" {
				//FIND_VALUE(key):
				println!("You have chosen to send a bid to a seller");
				println!("Bid value");
				line = String::new();
				stdin.read_line(&mut line).await?;
				let bid_value : i32 = line.trim().parse().unwrap();
				//list sellers we have subscribed to

				//choose seller

				//send bid to seller
				self.send_bid_to_seller(bid_value, "123".to_string(),"127.0.0.1".to_string(), "34000".to_string());
			} else {
				println!("Invalid Selection");
			}
		}
		Ok(())
	}


	fn ask_for_seller_list(&self) {
	
		//the basic idea is that we'll use FIND_COMP() to find a list of buyers
		
		//find all possible sellers
		let ip_list = self.find_all_possible_ips();
		let seller_info = self.return_seller_from_ips(ip_list);
		
		for (ip, port, item) in &seller_info {
			println!("{}:{} is a seller and it sells {}", ip, port, item);
		}

	}
	
	fn return_seller_from_ips(&self, list_of_ips : Vec<(String, String)>) -> Vec<(String, String, String)>{
		//use FIND_VALUE to see
		//if it's a seller and if the bid is active
		//what it sells
		
		//let mut list_of_sellers: Vec<(String, String)> = Vec::new();
		//seller info returned should be 
		//ip, port and item
		let mut seller_info: Vec<(String, String, String)> = Vec::new();
		
		for (ip, port) in &list_of_ips {
		
			let is_seller = task::block_on(self.node_client.FIND_VALUE(ip.clone(), port.clone(), self.node_client.node_IP.clone(), "666".to_string(), "is_seller".to_string()));
			let active_bid = task::block_on(self.node_client.FIND_VALUE(ip.clone(), port.clone(), self.node_client.node_IP.clone(), "666".to_string(), "active_bid".to_string()));
			
			//is_seller and active_bid should be in string format
			if is_seller == "true" && active_bid == "true"{
				let item = task::block_on(self.node_client.FIND_VALUE(ip.clone(), port.clone(), self.node_client.node_IP.clone(), "666".to_string(),"item".to_string()));
				let item = "item_placeholder".to_string();

				seller_info.push((ip.clone(),port.clone(),item));
				
			}
		}

		return seller_info;
	
	}
	
	fn find_all_possible_ips(&self) -> Vec<(String, String)>{
		//here we are going to use our FIND_COMP 
		let mut list_of_running_IPS: Vec<(String, String)> = Vec::new();

		let no_new_ip = false;
		
		//grab from routing table
		//unlock node from client and grab a copy of ids
		list_of_running_IPS = self.return_routing_table_as_port_ip_tuple();
		let mut list_of_total_IPS: Vec<(String, String)> = list_of_running_IPS.clone();
		
		
		while no_new_ip == true {
		//we run through our routing table and grab any new ips
			let mut new_ip_list = Vec::new();
			for (ip, port) in &list_of_running_IPS{
				//let ip = routing_table_pair.ip.clone();
				//let port = routing_table_pair.port.clone();
				let sha1_str = node_commands::sha1_to_string(&self.node_client.id);
				let send = task::block_on(self.node_client.FIND_COMP(ip.clone(), port.clone(), self.node_client.node_IP.clone(), "666".to_string(), self.node_client.id.clone()));
				let new_list = self.extract_ip_port_pairs(&send["comp_list"]);
				for (ip_new, port_new) in self.check_if_new_ip(&new_list, &list_of_total_IPS).iter(){
					new_ip_list.push((ip_new.to_string(), port_new.to_string()));
				}


			}
			
			list_of_running_IPS = new_ip_list;

			for (ip, port) in list_of_running_IPS.iter(){
				list_of_total_IPS.push((ip.to_string(), port.to_string()));
			}

		}

		return list_of_total_IPS;
	
	}

	fn extract_ip_port_pairs(&self, json: &JsonValue) -> Vec<(String, String)> {
		let mut ip_port_pairs = Vec::new();

		for item in json.members() {
			if let (Some(ip), Some(port)) = (item["ip"].as_str(), item["port"].as_str()) {
				ip_port_pairs.push((ip.to_string(), port.to_string()));
			}
		}


		ip_port_pairs
	}


	fn check_if_new_ip(&self, sublist: &[(String, String)], all_ips: &[(String, String)], ) -> Vec<(String, String)> {
		let existing_ips: Vec<(String, String)> = all_ips.iter().cloned().collect();
		let new_ips: Vec<(String, String)> = sublist
			.iter()
			.filter(|(ip, _)| !existing_ips.contains(&(ip.clone(), "".to_string())))
			.cloned()
			.collect();

		new_ips
	}


	fn return_routing_table_as_port_ip_tuple(&self) -> Vec<(String, String)> {
		let mut ret_list = Vec::new();
	
		let this_node = self.node_client.node.lock().unwrap();
		let routing_table_copy = this_node.routing_table.clone();
		drop(this_node);
		
		for (routing_table_pair) in &routing_table_copy{
		
			let ip = routing_table_pair.ip.clone();
			let port = routing_table_pair.port.clone();
			ret_list.push((ip,port));
		}
		
		return ret_list;
	}
	
	fn subscribe_to_buyer(&self, send_to_ip_of_buyer : String, send_to_port_of_buyer : String){
	
		let key = "buyer_ips".to_string();
		
		let value = format!("{}:{}", self.node_client.node_IP.clone(), self.node_client.node_port.clone());
		let mut ret_val = Vec::new();
		ret_val.push(value);
		//send store to buyer
		self.node_client.store(send_to_ip_of_buyer, send_to_port_of_buyer, key, StorageValue::Multiple(ret_val));
	
	
	}
	
	fn send_bid_to_seller(&self, bid_value : i32, wallet_id: String, send_to_ip : String, send_to_port : String){
	
		let key = "bids".to_string();
		let wallet_key = "wallet_id".to_string();
		let mut ret_bid = Vec::new();
		ret_bid.push(bid_value.to_string());

		let mut wallet_id_list = Vec::new();
		wallet_id_list.push(wallet_id);

		task::block_on(self.node_client.store(send_to_ip.clone(), send_to_port.clone(), key, StorageValue::Multiple(ret_bid)));
		task::block_on(self.node_client.store(send_to_ip.clone(), send_to_port.clone(), wallet_key, StorageValue::Multiple(wallet_id_list)));
	
	}
}

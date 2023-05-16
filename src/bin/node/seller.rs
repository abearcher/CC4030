
use crate::node::node_client;
use crate::node::node_object;
use async_std::{fs::File, io, prelude::*, task};
use async_std::net::UdpSocket;
use crate::node::node_object::StorageValue::Single;

pub struct Seller{
	node_client: node_client::NodeClient
	//list of subscribers?
}

//in seller's hashmap
//keys:
//1 - buyer_ips: this stores a list of buyers (subscribers) which shall be informed by the seller
//2 - bids : the bids registered for this sellers sale
//3 - is_seller: TRUE
//4 - active_bid: TRUE, until we close the bid
//5 - item_name: what we're selling!

impl Seller {

	pub fn new(node_client_in : node_client::NodeClient) -> Seller{

		return Seller{
			node_client: node_client_in,
		}

	}

	pub async fn command_selection(&self)-> io::Result<()> {
		let stdin = io::stdin();


		let mut line = String::new();
		loop{
			//let ip_of_sender = self.node_IP.clone();
			//let port_of_sender = "4949".to_string();
			println!("\nPlease select the following commands:\n1 - Start auction\n2 - end auction\n3 - print hashmap");
			println!("Selected {}", line);
			line = String::new();
			stdin.read_line(&mut line).await?;

			if line.trim().to_string() == "1"{
				println!("You have chosen to start the auction");
				println!("Please enter starting bid price");
				line = String::new();
				stdin.read_line(&mut line).await?;
				let starting_price = line.trim().to_string();
				println!("Please enter starting item name");
				line = String::new();
				stdin.read_line(&mut line).await?;
				let item_name = line.trim().to_string();
				self.start_auction(starting_price.clone(), item_name.clone());

			} else if line.trim().to_string() == "2"{
				//STORE(key, value);
				println!("You have chosen to end the auction.");
				self.end_auction();

			} else if line.trim().to_string() == "3" {
				//FIND_VALUE(key):
				println!("You have chosen to print storage. ");
				self.node_client.print_storage();
			} else {
				println!("Invalid Selection");
			}
		}
		Ok(())
	}

	fn start_auction(&self, starting_bid_price : String, item_name : String){
		println!("starting auction!");

		let ip = self.node_client.node_IP.clone();
		let port = self.node_client.node_port.clone();

		let bid_payload = Single(starting_bid_price);
		let item_name_payload = Single(item_name);
		let is_seller = Single("true".to_string());
		let is_active_bid = Single("true".to_string());

		task::block_on(self.node_client.store(ip.clone(), port.clone(), "is_seller".to_string(), is_seller));
		task::block_on(self.node_client.store(ip.clone(), port.clone(), "bids".to_string(), bid_payload));
		task::block_on(self.node_client.store(ip.clone(), port.clone(), "item_name".to_string(), item_name_payload));
		task::block_on(self.node_client.store(ip.clone(), port.clone(), "active_bid".to_string(), is_active_bid));
	}

	fn end_auction(&self){
			//for every seller IP
			let list_of_ips = self.node_client.ret_storage(&"buyer_ips");

			// Check if the storage value is of type StorageValue::Multiple
			if let Some(node_object::StorageValue::Multiple(values)) = list_of_ips {
				for ip_port in values {
					// Store the value to the client
					// Key: ID of the seller
					// Value: New bid info
					let (send_to_ip, send_to_port) = self.node_client.parse_ip_port(ip_port);
					let key = "ongoing_auction".to_string();
					let value = "false".to_string();

					self.node_client.store(send_to_ip, send_to_port, key, node_object::StorageValue::Single(value));
				}
			} else {
				println!("Auction does not exist");
			}
	
		//add to blockchain!!!!!
	}
}

use std::{thread, time};
use std::time::Duration;
use async_std::task;

use crate::node::node_object;
use crate::node::node_commands;


pub(crate) struct AuctionTimer {
    auction_ip: String,
    auction_port: String,
}


impl AuctionTimer {
    pub fn new(ip_in: String, port_in : String) -> AuctionTimer{
        return AuctionTimer{
            auction_ip: ip_in,
            auction_port: port_in,
        };
    }

    pub async fn auction_timer(&self, seconds : u64){
        let duration = Duration::from_secs(seconds);
        thread::sleep(duration);
        task::block_on(self.store(self.auction_ip.clone(), self.auction_port.clone(),
                                  "active_auction".to_string(), node_object::StorageValue::Single("false".to_string()
        )));
    }

    async fn store(&self, send_to_ip: String, send_to_port: String, key: String, value: node_object::StorageValue) {
        let value_payload = match value {
            node_object::StorageValue::Single(s) => json::JsonValue::String(s),
            node_object::StorageValue::Multiple(vec) => json::JsonValue::Array(
                vec.into_iter().map(json::JsonValue::String).collect(),
            ),
        };

        let store_payload = json::object!(
			"key": key,
			"value": value_payload,
		);

        let store_cmd = node_commands::craft_command("STORE".to_string(), store_payload);

        task::block_on(node_commands::send_command_to_node(send_to_ip, send_to_port, store_cmd));
    }


}
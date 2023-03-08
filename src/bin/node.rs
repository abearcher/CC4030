use async_std::net::UdpSocket;
use futures::executor::block_on;


pub struct Node {
	pub ip_list: Vec<String>,
	pub node_port: String, 
	pub node_IP: String,
}

impl Node {
	pub fn start_first_node(&self) -> std::io::Result<()> {
		{
			block_on(self.run_node_server());

		} // the socket is closed here
			Ok(())
	}
	
	pub fn start_sub_node(&self, known_ip: String, known_port: String)  -> std::io::Result<()> {
		{
			println!("connecting with known node!");
			block_on(self.connect_with_node(known_ip, known_port));
			println!("starting own server");
			//block_on(self.run_node_server());
			println!("end!")
		}
			Ok(())
	}
	
	async fn connect_with_node(&self, known_ip: String, known_port: String) -> std::io::Result<()> {
		
		const THE_MERCHANT_OF_VENICE: &[u8] = b"
		    If you prick us, do we not bleed?
		    If you tickle us, do we not laugh?
		    If you poison us, do we not die?
		    And if you wrong us, shall we not revenge?
		";

		let socket = UdpSocket::bind("127.0.0.1:0").await?;

		let addr = "127.0.0.1:34254";
		let sent = socket.send_to(THE_MERCHANT_OF_VENICE, &addr).await?;
		println!("Sent {} bytes to {}", sent, addr);
		Ok(())
	}
	
	async fn run_node_server(&self) -> std::io::Result<()> {
		let socket = UdpSocket::bind("127.0.0.1:34254").await?;
		println!("Starting server");
		
		let mut buf = vec![0u8; 1024];
		
		loop {
			let (n, peer) = socket.recv_from(&mut buf).await?;
			socket.send_to(&buf[..n], &peer).await?;
			println!("Received {} bytes from {}", n, peer);
			println!("Received message was {:?}", String::from_utf8_lossy(&buf));
			
			//println!("stuff inside code");
		}
	}
	
	async fn communicate_with_client(socket: UdpSocket){
		
		
	    //receive_msg(socket);
	    //send_IPs(socket);
	}
	
	
	async fn send_IPs(socket: UdpSocket){

	}

	async fn receive_msg(socket: UdpSocket) -> std::io::Result<()> {
		let mut buf = vec![0; 1024];
		let (n, peer) = socket.recv_from(&mut buf).await?;
		println!("Received {} bytes from {}", n, peer);
	    //let mut buf = [0; 10];
	    //let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)
		//.expect("Didn't receive data");
	    //let filled_buf = &mut buf[..number_of_bytes];
	    	Ok(())
	}
}

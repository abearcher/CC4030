async fn command_selection(&self)-> io::Result<()> {
	let stdin = io::stdin();
	let mut line = String::new();
	loop{
		line = String::new();
		stdin.read_line(&mut line).await?;
		println!("Please select the following commands:\n1-PING Computer\n2- GET value\n3 - STORE value");
		println!("Selected {}", line);
		if line == "1"{
			println!("You have chosen to PING. Please select IP");
			line = String::new();
			stdin.read_line(&mut line).await?;
			let ip = line;
			line = String::new();
			println!("Please enter port");
			stdin.read_line(&mut line).await?;
			let port = line;
							
			//self.full_ping_cmd(ip, port);
			
		} else if line == "2"{
			//STORE();
		} else if line == "3" {
			//GET():
		} else {
			println!("Invalid Selection");
		}
	}
	Ok(())
}

use std::io;

mod models;

fn main() {

    println!("{}", r"
   /$$   /$$  /$$$$$$   /$$$$$$   /$$$$$$          /$$                       /$$                              
  | $$  | $$ /$$$_  $$ /$$__  $$ /$$$_  $$        | $$                      | $$                              
  | $$  | $$| $$$$\ $$|__/  \ $$| $$$$\ $$        | $$        /$$$$$$   /$$$$$$$  /$$$$$$   /$$$$$$   /$$$$$$ 
  | $$$$$$$$| $$ $$ $$   /$$$$$/| $$ $$ $$ /$$$$$$| $$       /$$__  $$ /$$__  $$ /$$__  $$ /$$__  $$ /$$__  $$
  |_____  $$| $$\ $$$$  |___  $$| $$\ $$$$|______/| $$      | $$$$$$$$| $$  | $$| $$  \ $$| $$$$$$$$| $$  \__/
        | $$| $$ \ $$$ /$$  \ $$| $$ \ $$$        | $$      | $$_____/| $$  | $$| $$  | $$| $$_____/| $$      
        | $$|  $$$$$$/|  $$$$$$/|  $$$$$$/        | $$$$$$$$|  $$$$$$$|  $$$$$$$|  $$$$$$$|  $$$$$$$| $$      
        |__/ \______/  \______/  \______/         |________/ \_______/ \_______/ \____  $$ \_______/|__/      
                                                                                 /$$  \ $$                    
                                                                                |  $$$$$$/                    
                                                                                 \______/                     
  ");

    // Print the options to the user
    println!("Welcome to our ledger system! Choose an option:");
    println!("1. Create a new wallet");
    println!("2. Print your wallet keys");
    println!("3. Sign message");
    println!("4. Initialize blockchain");

    // Read the user's input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Parse the input into a number
    let choice: u32 = input.trim().parse().unwrap();

    // Match the number to the corresponding option
    match choice {
        1 => {println!("Insert the owner email please:");
            let mut email = String::new();
            io::stdin().read_line(&mut email).unwrap();
            // create the wallet
            let new_wallet = models::wallet::Wallet::new(email.to_string());
            models::wallet::Wallet::sign_data(email.to_string(),new_wallet.keypair.1);
            // for debug purposes only
            //println!("{:?}", new_wallet);
        },

        2 => println!("You chose Option 2"),//models::wallet::Wallet::debug_print_keys(new_wallet.keypair)

        3 => println!("You chose Option 3"),

        4 => {let difficulty = 1;
            let mut transaction = "Transaction test!";
            let mut blockchain = models::blockchain::Blockchain::new(difficulty);
            models::blockchain::Blockchain::add_block(&mut blockchain, transaction.to_string());
        },
        _ => println!("Invalid option"),
    }
}
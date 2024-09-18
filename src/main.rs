mod block;
mod blockchain;

use blockchain::{Blockchain, Transaction};
use std::io::{self, Write};

fn main() {
    let mut blockchain = Blockchain::new(2);

    loop {
        println!("\nBlockchain Menu:");
        println!("1. Add a new transaction");
        println!("2. Mine pending transactions");
        println!("3. View the blockchain");
        println!("4. Validate the blockchain");
        println!("5. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => add_transaction(&mut blockchain),
            "2" => mine_transactions(&mut blockchain),
            "3" => view_blockchain(&blockchain),
            "4" => validate_blockchain(&blockchain),
            "5" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn add_transaction(blockchain: &mut Blockchain) {
    print!("Enter sender: ");
    io::stdout().flush().unwrap();
    let mut sender = String::new();
    io::stdin()
        .read_line(&mut sender)
        .expect("Failed to read line");

    print!("Enter recipient: ");
    io::stdout().flush().unwrap();
    let mut recipient = String::new();
    io::stdin()
        .read_line(&mut recipient)
        .expect("Failed to read line");

    print!("Enter amount: ");
    io::stdout().flush().unwrap();
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read line");
    let amount: f64 = amount.trim().parse().expect("Please enter a valid number");

    let transaction = Transaction::new(
        sender.trim().to_string(),
        recipient.trim().to_string(),
        amount,
    );
    blockchain.add_transaction(transaction);
    println!("Transaction added successfully!");
}

fn mine_transactions(blockchain: &mut Blockchain) {
    print!("Enter miner address: ");
    io::stdout().flush().unwrap();
    let mut miner_address = String::new();
    io::stdin()
        .read_line(&mut miner_address)
        .expect("Failed to read line");

    blockchain.mine_pending_transactions(miner_address.trim().to_string());
    println!("Pending transactions have been mined!");
}

fn view_blockchain(blockchain: &Blockchain) {
    println!("\nBlockchain:");
    for (i, block) in blockchain.chain.iter().enumerate() {
        println!("Block {}:", i);
        println!("  Index: {}", block.index);
        println!("  Timestamp: {}", block.timestamp);
        println!("  Proof: {}", block.proof);
        println!("  Previous hash: {}", block.previous_hash);
        println!("  Transactions:");
        for transaction in &block.transactions {
            println!(
                "    From: {} To: {} Amount: {}",
                transaction.sender, transaction.recipient, transaction.amount
            );
        }
        println!("  Hash: {}", block.calculate_hash());
    }
}

fn validate_blockchain(blockchain: &Blockchain) {
    if blockchain.is_chain_valid() {
        println!("The blockchain is valid.");
    } else {
        println!("The blockchain is NOT valid!");
    }
}

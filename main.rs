use ic_cdk_macros::{update, query, init};
use ic_principal::Principal;
use candid::CandidType;
use serde::{Serialize, Deserialize};
use std::cell::RefCell;
use std::io::{self, Write};
use std::result::Result;

#[derive(Default, Serialize, Deserialize, CandidType)]
struct Wallet {
    balance: u64,
}

thread_local! {
    // Global storage for the wallet.
    static WALLET: RefCell<Wallet> = RefCell::new(Wallet::default());
}

#[init]
fn init_wallet() {
    // Initialize the wallet with a default balance, e.g., 100 tokens.
    WALLET.with(|wallet| {
        wallet.borrow_mut().balance = 100;
    });
}

#[update]
fn send_tokens(amount: u64, to: Principal) -> Result<String, String> {
    WALLET.with(|wallet| {
        let mut wallet = wallet.borrow_mut();
        if wallet.balance < amount {
            return Err("Insufficient balance.".to_string());
        }
        
        wallet.balance -= amount;
        ic_cdk::println!("Sent {} tokens to {:?}", amount, to);
        
        Ok(format!("Sent {} tokens to {:?}", amount, to))
    })
}

#[update]
fn receive_tokens(amount: u64) -> String {
    WALLET.with(|wallet| {
        let mut wallet = wallet.borrow_mut();
        wallet.balance += amount;

        ic_cdk::println!("Received {} tokens", amount);
        format!("Received {} tokens", amount)
    })
}

#[query]
fn get_balance() -> u64 {
    WALLET.with(|wallet| wallet.borrow().balance)
}

fn main() {
    init_wallet(); // Initialize the wallet on startup.
    
    loop {
        println!("\n--- Token Wallet ---");
        println!("1. Check Balance");
        println!("2. Send Tokens");
        println!("3. Receive Tokens");
        println!("4. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately.

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let balance = get_balance();
                println!("Current Balance: {} tokens", balance);
            },
            "2" => {
                let mut amount_input = String::new();
                print!("Enter amount to send: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut amount_input).expect("Failed to read line");
                let amount: u64 = amount_input.trim().parse().expect("Invalid amount");

                let to = Principal::anonymous(); // Replace with actual Principal input if needed.
                match send_tokens(amount, to) {
                    Ok(msg) => println!("{}", msg),
                    Err(err) => println!("Error: {}", err),
                }
            },
            "3" => {
                let mut amount_input = String::new();
                print!("Enter amount to receive: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut amount_input).expect("Failed to read line");
                let amount: u64 = amount_input.trim().parse().expect("Invalid amount");

                let msg = receive_tokens(amount);
                println!("{}", msg);
            },
            "4" => {
                println!("Exiting...");
                break;
            },
            _ => println!("Invalid option, please try again."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    #[test]
    fn test_send_tokens() {
        WALLET.with(|wallet| wallet.borrow_mut().balance = 50);
        let to = Principal::anonymous();
        assert_eq!(send_tokens(20, to), Ok(format!("Sent 20 tokens to {:?}", to)));
        assert_eq!(get_balance(), 30);
    }

    #[test]
    fn test_receive_tokens() {
        WALLET.with(|wallet| wallet.borrow_mut().balance = 50);
        receive_tokens(20);
        assert_eq!(get_balance(), 70);
    }
}

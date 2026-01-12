use crate::utils::hash::calculate_hash;
use super::transaction::Transaction;
use std::collections::HashMap;
use super::block::Block;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub accounts: HashMap<String, u32>
}

impl Blockchain {
    pub fn new_blockchain() -> Self {
        let mut initial_accounts = HashMap::new();

        initial_accounts.insert(String::from("Alice"), 100);
        initial_accounts.insert(String::from("Bob"), 50);
        initial_accounts.insert(String::from("Moi"), 75);

        let genesis_block = Block::new_block(0, String::from("0"), vec![]);
        
        Blockchain {
            chain: vec![genesis_block],
            accounts: initial_accounts,
        }
    }

    pub fn add_block(&mut self, data: Vec<Transaction>) {
        let dernier_bloc = self.chain.last().expect("La chaîne ne doit pas être vide");
        
        let nouveau_bloc = Block::new_block(
            dernier_bloc.index + 1,
            dernier_bloc.hash.clone(),
            data,
        );
        
        self.chain.push(nouveau_bloc);
    }

    pub fn apply_block(&mut self, block: &Block) {
        for tx in &block.data {
            if let Some(balance) = self.accounts.get_mut(&tx.sender) {
                *balance = balance.checked_sub(tx.amount).unwrap_or(0);
            }
            
            if let Some(receiver_balance) = self.accounts.get_mut(&tx.receiver) {
                *receiver_balance += tx.amount;
            } else {
                println!("Transaction invalide : l'adresse du receveur est inconnue. Les fonds ont été définitivement brûlés.");
            }
        }
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash != calculate_hash(current) || current.prev_hash != previous.hash {
                return false;
            }
        }
        true
    }

    pub fn verify_transaction(&self, tx: &Transaction) -> (bool, String) {
        if let Some(balance) = self.accounts.get(&tx.sender) {
            return (*balance >= tx.amount, tx.sender.clone());
        }
        (false, tx.sender.clone())
    }

    pub fn display_status(&self, title: &str) {
        println!("\n\x1b[1;35m--- {} ---\x1b[0m", title);
        println!("┌──────────────────────┬─────────────┐");
        println!("│      UTILISATEUR     │    SOLDE    │");
        println!("├──────────────────────┼─────────────┤");
        let mut sorted_accounts: Vec<_> = self.accounts.iter().collect();
        sorted_accounts.sort_by(|a, b| a.0.cmp(b.0));
        for (user, balance) in sorted_accounts {
            println!("│ {:<20} │ {:>7} BTC │", user, balance);
        }
        println!("└──────────────────────┴─────────────┘");
    }
}
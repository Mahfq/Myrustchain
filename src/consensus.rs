use crate::blockchain::{Block, Blockchain};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum ConsensusMessage {
    PrePrepare { block: Block, view: u32 },
    Prepare { block_hash: String, node_id: u32 },
    Commit { block_hash: String, node_id: u32 },
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u32,
    pub blockchain: Blockchain,
    pub messages_received: Vec<ConsensusMessage>,
}

impl Node {
    pub fn new(id: u32) -> Self {
        Node {
            id,
            blockchain: Blockchain::new_blockchain(),
            messages_received: Vec::new(),
        }
    }

pub fn receive_message(&mut self, msg: ConsensusMessage) -> Option<ConsensusMessage> {
        self.messages_received.push(msg.clone());

        match msg {
            ConsensusMessage::PrePrepare { block, view: _ } => {
                if self.is_block_valid(&block) {
                    Some(ConsensusMessage::Prepare { 
                        block_hash: block.hash.clone(), 
                        node_id: self.id 
                    })
                } else {
                    None
                }
            },

            ConsensusMessage::Prepare { block_hash, .. } => {
                if self.count_unique_votes(&block_hash, "prepare") == 3 {
                    Some(ConsensusMessage::Commit { 
                        block_hash: block_hash.clone(), 
                        node_id: self.id 
                    })
                } else {
                    None
                }
            },

            ConsensusMessage::Commit { block_hash, .. } => {
                if self.count_unique_votes(&block_hash, "commit") >= 3 {
                    if let Some(block) = self.find_block_in_messages(&block_hash) {
                        if !self.blockchain.chain.iter().any(|b| b.hash == block.hash) {
                            
                            self.blockchain.apply_block(&block);
                            self.blockchain.chain.push(block);
                            self.messages_received.clear();
                        }
                    }
                }
                None
            },
        }
    }

    fn is_block_valid(&self, block: &Block) -> bool{
        if block.prev_hash != self.blockchain.chain.last().unwrap().hash {
            return false ;
        }

        let mut temp_balances = self.blockchain.accounts.clone();

        for tx in &block.data {
                let sender_balance = temp_balances.get(&tx.sender).cloned().unwrap_or(0);
                if sender_balance < tx.amount {
                    println!("  [Nœud {}] ❌ Bloc rejeté : {} tente de dépenser {} mais n'a plus que {} (cumulé) !", self.id, tx.sender, tx.amount, sender_balance);
                    return false;
                }
                
                if let Some(balance) = temp_balances.get_mut(&tx.sender) {
                    *balance -= tx.amount;
                }
                
                let receiver_balance = temp_balances.entry(tx.receiver.clone()).or_insert(0);
                *receiver_balance += tx.amount;
            }
        true
    }

    fn find_block_in_messages(&self, hash: &String) -> Option<Block> {
        self.messages_received.iter().find_map(|m| {
            if let ConsensusMessage::PrePrepare { block, .. } = m {
                if &block.hash == hash {
                    return Some(block.clone());
                }
            }
            None
        })
    }

    fn count_unique_votes(&self, hash: &String, msg_type: &str) -> usize {
        let mut voters = HashSet::new();
        for m in &self.messages_received {
            match m {
                ConsensusMessage::Prepare { block_hash, node_id } if msg_type == "prepare" => {
                    if block_hash == hash { voters.insert(node_id); }
                },
                ConsensusMessage::Commit { block_hash, node_id } if msg_type == "commit" => {
                    if block_hash == hash { voters.insert(node_id); }
                },
                _ => {}
            }
        }
        voters.len()
    }
}
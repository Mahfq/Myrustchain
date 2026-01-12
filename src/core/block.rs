use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::hash::calculate_hash;
use super::transaction::Transaction; 

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub data: Vec<Transaction>,
    pub prev_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new_block(index: u32, prev_hash: String, data: Vec<Transaction>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut block = Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash: String::new(),
        };

        block.hash = calculate_hash(&block);
        block
    }
}
use sha2::{Sha256, Digest};
use crate::core::block::Block;

pub fn calculate_hash(block: &Block) -> String {
    let mut hasher = Sha256::new();
    
    let tx_string: String = block.data
        .iter()
        .map(|tx| tx.to_string())
        .collect::<Vec<String>>()
        .join("");
    
    let input = format!("{}{}{}{}", block.index, block.timestamp, tx_string, block.prev_hash);
    
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}
use crate::core::block::Block;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusMessage {
    PrePrepare { block: Block, view: u32 },
    Prepare { block_hash: String, node_id: u32 },
    Commit { block_hash: String, node_id: u32 },
}
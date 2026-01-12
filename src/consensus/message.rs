use crate::core::block::Block;

#[derive(Debug, Clone)]
pub enum ConsensusMessage {
    PrePrepare { block: Block, view: u32 },
    Prepare { block_hash: String, node_id: u32 },
    Commit { block_hash: String, node_id: u32 },
}
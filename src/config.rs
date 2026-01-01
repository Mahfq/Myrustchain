pub struct Config {
    pub total_nodes: usize,
    pub block_timeout_ms: u64,
}

impl Config {
    pub fn quorum_size(&self) -> usize {
        let f = (self.total_nodes - 1) / 3;
        2 * f + 1
    }

    pub fn get_leader_id(&self, block_index: u32) -> u32 {
        block_index % self.total_nodes as u32
    }
}
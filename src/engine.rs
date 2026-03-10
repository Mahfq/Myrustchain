use crate::consensus::message::ConsensusMessage;
use crate::core::transaction::Transaction;
use crate::consensus::pbft::Node;
use crate::core::block::Block;
use crate::config::Config;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use std::time::Duration;
use tokio::time::sleep;
use tokio::spawn;
use rand::Rng;

pub struct ConsensusEngine {
    pub nodes: Vec<Node>,
    pub config: Config,
}

impl ConsensusEngine {
    pub async fn new(config: Config) -> Self {
        let mut nodes = Vec::new();
        let base_port = 8000;

        for id in 0..config.total_nodes {
            let node = Node::new(id as u32);
            let port = base_port + id as u16;
            
            nodes.push(node.clone());

            let node_for_server = node.clone();
            let quorum = config.quorum_size();
            let total_nodes = config.total_nodes;

            spawn(async move {
                node_for_server.start_server(port, quorum, total_nodes).await;
            });
        }

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        Self { nodes, config }
    }

    pub async fn run_next_cycle(&mut self, block_index: u32) -> bool{
        let total_nodes = self.config.total_nodes;
        let quorum = self.config.quorum_size();

        let leader_id = block_index as usize % total_nodes;
        println!("\n\x1b[1;34m━━━━━━━━━━━━━━━ CYCLE : BLOC #{} ━━━━━━━━━━━━━━━\x1b[0m", block_index);
        log::info!("👑 Leader actuel : Nœud {} (Quorum requis: {})", leader_id, quorum);

        let mut rng = rand::rng();
        let users = vec!["Alice", "Bob", "Moi"];
        let nb_tx = rng.random_range(1..=3);
        let mut txs = Vec::new();

        println!("📝 Transactions proposées :");
        for _ in 0..nb_tx {
            let sender = users[rng.random_range(0..users.len())];
            let mut receiver = users[rng.random_range(0..users.len())];
            while receiver == sender { receiver = users[rng.random_range(0..users.len())]; }
            let amount = rng.random_range(0..10);
            println!("   • {} -> {} ({} BTC)", sender, receiver, amount);
            txs.push(Transaction { sender: sender.into(), receiver: receiver.into(), amount });
        }

        let last_hash = self.nodes[leader_id].blockchain.chain.last().unwrap().hash.clone();
        let new_block = Block::new_block(block_index, last_hash, txs);
        let pp_msg = ConsensusMessage::PrePrepare { block: new_block.clone(), view: 0 };
        
        log::info!("\n\x1b[33m[1/3] PRE-PREPARE :\x1b[0m Le Leader diffuse la proposition via TCP...");
        
        let bytes = bincode::serialize(&pp_msg).unwrap();
        let base_port = 8000;

        for i in 0..total_nodes {
            let port = base_port + i as u16;
            let addr = format!("127.0.0.1:{}", port);
            let bytes_clone = bytes.clone();

            tokio::spawn(async move {
                if let Ok(mut stream) = TcpStream::connect(&addr).await {
                    let _ = stream.write_all(&bytes_clone).await;
                }
            });
        }

        sleep(Duration::from_millis(self.config.block_timeout_ms)).await;

        for node in self.nodes.iter_mut() {
            node.blockchain.apply_block(&new_block);
            node.blockchain.chain.push(new_block.clone());
        }

        true
    }   
}
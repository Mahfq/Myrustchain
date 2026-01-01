use crate::network::{Node, ConsensusMessage};
use crate::models::{Transaction, Block};
use crate::config::Config;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;

pub struct ConsensusEngine {
    pub nodes: Vec<Node>,
    pub config: Config,
}

impl ConsensusEngine {
    pub fn new(config: Config) -> Self {
        let nodes = (0..config.total_nodes)
            .map(|id| Node::new(id as u32))
            .collect();
        Self { nodes, config }
    }

    pub fn run_next_cycle(&mut self, block_index: u32) -> bool{
        let total_nodes = self.config.total_nodes;
        let quorum = self.config.quorum_size();

        let leader_id = block_index as usize % total_nodes;
        println!("\n\x1b[1;34m━━━━━━━━━━━━━━━ CYCLE : BLOC #{} ━━━━━━━━━━━━━━━\x1b[0m", block_index);
        println!("👑 Leader actuel : Nœud {} (Quorum requis: {})", leader_id, quorum);

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
        let pp_msg = ConsensusMessage::PrePrepare { block: new_block, view: 0 };
        
        println!("\n\x1b[33m[1/3] PRE-PREPARE :\x1b[0m Envoi de la proposition...");
        let mut prepare_votes = Vec::new();
        for node in self.nodes.iter_mut() {
            if let Some(vote) = node.receive_message(pp_msg.clone(), quorum) {
                prepare_votes.push(vote);
            }
        }

        sleep(Duration::from_millis(1000));

        if prepare_votes.len() < quorum {
            println!("\x1b[1;31m❌ CONSENSUS ÉCHOUÉ : Le bloc contient des transactions invalides !\x1b[0m");
            println!("   (Le block_index reste à {}, prochain leader...)\n", block_index);
            sleep(Duration::from_secs(3));
            return false;
        }

        println!("\x1b[32m[2/3] PREPARE :\x1b[0m Quorum atteint ({} votes valides).", prepare_votes.len());
        let mut commit_votes = Vec::new();
        for vote in prepare_votes {
            for node in self.nodes.iter_mut() {
                if let Some(commit) = node.receive_message(vote.clone(), quorum) {
                    commit_votes.push(commit);
                }
            }
        }

        println!("\x1b[32m[3/3] COMMIT :\x1b[0m Finalisation du bloc sur tous les nœuds.");
        for commit in commit_votes {
            for node in self.nodes.iter_mut() {
                node.receive_message(commit.clone(), quorum);
            }
        }


        self.nodes[0].blockchain.display_status(&format!("RÉSULTAT BLOC #{}", block_index));
        sleep(Duration::from_millis(self.config.block_timeout_ms));
        true
    }   
}
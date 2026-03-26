use super::message::ConsensusMessage;
use crate::core::chain::Blockchain;
use crate::core::block::Block;
use std::collections::HashSet;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

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

    pub async fn broadcast_message(&self, msg: ConsensusMessage, total_nodes: usize) {
        let bytes = match bincode::serialize(&msg) {
            Ok(b) => b,
            Err(e) => {
                log::error!("❌ Erreur de sérialisation : {}", e);
                return;
            }
        };

        let base_port = 8000;

        for i in 0..total_nodes {
            if i as u32 == self.id {
                continue; 
            }

            let port = base_port + i as u16;
            let addr = format!("127.0.0.1:{}", port);
            let bytes_clone = bytes.clone(); 

            tokio::spawn(async move {
                match TcpStream::connect(&addr).await {
                    Ok(mut stream) => {
                        if let Err(e) = stream.write_all(&bytes_clone).await {
                            log::error!("Erreur d'écriture TCP vers {} : {}", addr, e);
                        }
                    }
                    Err(e) => {
                        log::debug!("Impossible de joindre le nœud sur {} : {}", addr, e);
                    }
                }
            });
        }
    }

    pub fn receive_message(&mut self, msg: ConsensusMessage, quorum: usize) -> Option<ConsensusMessage> {
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
                if self.count_unique_votes(&block_hash, "prepare") >= quorum {
                    Some(ConsensusMessage::Commit { 
                        block_hash: block_hash.clone(), 
                        node_id: self.id 
                    })
                } else {
                    None
                }
            },

            ConsensusMessage::Commit { block_hash, .. } => {
                if self.count_unique_votes(&block_hash, "commit") >= quorum {
                    if let Some(block) = self.find_block_in_messages(&block_hash) {
                        if !self.blockchain.chain.iter().any(|b| b.hash == block.hash) {
                            
                            self.blockchain.apply_block(&block);
                             self.blockchain.chain.push(block.clone());
                            self.messages_received.clear();

                            log::info!("✅ Nœud {} a définitivement validé le BLOC #{}", self.id, block.index);
                            
                            if self.id == 0 {
                                self.blockchain.display_status(&format!("RÉSULTAT BLOC #{} (Vu par Nœud 0)", block.index));
                            }
                        }
                    }
                }
                None
            },
        }
    }

    fn is_block_valid(&self, block: &Block) -> bool{
        if block.prev_hash != self.blockchain.chain.last().unwrap().hash {
            log::warn!("  [Nœud {}] ❌ Bloc #{} rejeté : Le prev_hash ne correspond pas à notre dernier bloc !", self.id, block.index);
            return false;
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

    pub async fn start_server(mut self, port: u16, quorum: usize, total_nodes: usize) {
        let addr = format!("127.0.0.1:{}", port);
        
        let listener = match TcpListener::bind(&addr).await {
            Ok(l) => l,
            Err(e) => {
                log::error!("❌ Nœud {} n'a pas pu ouvrir le port {} : {}", self.id, port, e);
                return;
            }
        };

        log::info!("🟢 Nœud {} est en ligne et écoute sur {}", self.id, addr);

        loop {
            match listener.accept().await {
                Ok((mut socket, peer_addr)) => {
                    
                    let mut buffer = Vec::new();
                    match socket.read_to_end(&mut buffer).await {
                        Ok(_) => {
                            match bincode::deserialize::<ConsensusMessage>(&buffer) {
                                Ok(msg) => {
                                    log::debug!("Nœud {} a reçu un message de {}", self.id, peer_addr);
                                    
                                    if let Some(reponse) = self.receive_message(msg, quorum) {
                                        self.broadcast_message(reponse, total_nodes).await;
                                    }
                                },
                                Err(e) => log::error!("Erreur de désérialisation bincode : {}", e),
                            }
                        },
                        Err(e) => log::error!("Erreur de lecture TCP : {}", e),
                    }
                }
                Err(e) => log::error!("Erreur de connexion entrante : {}", e),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::transaction::Transaction;
    use crate::core::block::Block;

    fn mock_block(index: u32, prev_hash: String) -> Block {
        Block::new_block(index, prev_hash, vec![
            Transaction {
                sender: "Alice".to_string(),
                receiver: "Bob".to_string(),
                amount: 10,
            }
        ])
    }

    #[test]
    fn test_node_init() {
        let node = Node::new(0);
        assert_eq!(node.id, 0);
        assert_eq!(node.blockchain.chain.len(), 1);
    }

    #[test]
    fn test_invalid_prev_hash() {
        let mut node = Node::new(1);
        let bad_block = mock_block(1, "wrong_hash".to_string());
        let msg = ConsensusMessage::PrePrepare { block: bad_block, view: 0 };
        
        assert!(node.receive_message(msg, 1).is_none());
    }

    #[test]
    fn test_double_spend_protection() {
        let mut node = Node::new(1);
        let txs = vec![
            Transaction { sender: "Alice".to_string(), receiver: "Bob".to_string(), amount: 80 },
            Transaction { sender: "Alice".to_string(), receiver: "Moi".to_string(), amount: 40 },
        ];
        let block = Block::new_block(1, node.blockchain.chain[0].hash.clone(), txs);
        
        let msg = ConsensusMessage::PrePrepare { block, view: 0 };
        assert!(node.receive_message(msg, 1).is_none());
    }

    #[test]
    fn test_quorum_logic() {
        let mut node = Node::new(1);
        let hash = "test_hash".to_string();
        let quorum = 2;

        node.receive_message(ConsensusMessage::Prepare { block_hash: hash.clone(), node_id: 2 }, quorum);
        let res = node.receive_message(ConsensusMessage::Prepare { block_hash: hash.clone(), node_id: 3 }, quorum);
        
        assert!(matches!(res, Some(ConsensusMessage::Commit { .. })));
    }
}
use myrustchain::blockchain::{Transaction, Block};
use myrustchain::consensus::{Node, ConsensusMessage};
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;

fn main() {
    let mut nodes: Vec<Node> = (0..4).map(|id| Node::new(id)).collect();
    let mut block_index = 1;
    let mut rng = rand::rng();

    let users = vec!["Alice", "Bob", "Moi"];
    nodes[0].blockchain.display_status("ÉTAT INITIAL DU RÉSEAU");


    println!("==================================================");
    println!("🚀 LANÇEMENT DU RÉSEAU MYRUSTCHAIN (avec 4 nœuds)");
    println!("==================================================");
    
    loop {
        let leader_id = (block_index % 4) as usize;
        println!("\n\x1b[1;34m━━━━━━━━━━━━━━━ NOUVEAU CYCLE : BLOC #{} ━━━━━━━━━━━━━━━\x1b[0m", block_index);
        println!("👑 Leader actuel : Nœud {}", leader_id);

        let nb_tx = rng.random_range(1..=3);
        let mut txs = Vec::new();

        println!("📝 Transactions proposées :");
        for _ in 0..nb_tx {
            let sender = users[rng.random_range(0..users.len())];
            let mut receiver = users[rng.random_range(0..users.len())];
            while receiver == sender { receiver = users[rng.random_range(0..users.len())]; }
            let amount = rng.random_range(10..50);
            println!("   • {} -> {} ({} BTC)", sender, receiver, amount);
            txs.push(Transaction { sender: sender.into(), receiver: receiver.into(), amount });
        }

        let last_hash = nodes[leader_id].blockchain.chain.last().unwrap().hash.clone();
        let new_block = Block::new_block(block_index, last_hash, txs);
        let pp_msg = ConsensusMessage::PrePrepare { block: new_block, view: 0 };
        
        println!("\n\x1b[33m[1/3] PRE-PREPARE :\x1b[0m Envoi de la proposition...");
        let mut prepare_votes = Vec::new();
        for node in nodes.iter_mut() {
            if let Some(vote) = node.receive_message(pp_msg.clone()) {
                prepare_votes.push(vote);
            }
        }

        sleep(Duration::from_millis(1000));

        if prepare_votes.len() < 3 {
            println!("\x1b[1;31m❌ CONSENSUS ÉCHOUÉ : Le bloc contient des transactions invalides !\x1b[0m");
            println!("   (Le block_index reste à {}, prochain leader...)\n", block_index);
            sleep(Duration::from_secs(3));
            continue;
        }

        println!("\x1b[32m[2/3] PREPARE :\x1b[0m Quorum atteint ({} votes valides).", prepare_votes.len());
        let mut commit_votes = Vec::new();
        for vote in prepare_votes {
            for node in nodes.iter_mut() {
                if let Some(commit) = node.receive_message(vote.clone()) {
                    commit_votes.push(commit);
                }
            }
        }

        println!("\x1b[32m[3/3] COMMIT :\x1b[0m Finalisation du bloc sur tous les nœuds.");
        for commit in commit_votes {
            for node in nodes.iter_mut() {
                node.receive_message(commit.clone());
            }
        }


        nodes[0].blockchain.display_status(&format!("RÉSULTAT BLOC #{}", block_index));
        block_index += 1; 
        sleep(Duration::from_secs(5));
    }
}
use myrustchain::engine::ConsensusEngine;
use myrustchain::config::Config;
use rand::Rng;

fn main() {
    let nb_nodes = rand::rng().random_range(3..=10);
    let config = Config { total_nodes: nb_nodes, block_timeout_ms: 1000};

    println!("==================================================");
    println!("🚀 LANÇEMENT DU RÉSEAU MYRUSTCHAIN (avec {} nœuds)", config.total_nodes);
    println!("==================================================");

    let mut engine = ConsensusEngine::new(config);
    let mut block_index = 1;

    loop {
        let success = engine.run_next_cycle(block_index);

        if success {
            block_index += 1;
        } else {
            println!("🔄 Nouvelle tentative pour le bloc {}", block_index);
        }
    }
}
use myrustchain::engine::ConsensusEngine;
use myrustchain::config::Config;
use rand::Rng;

#[tokio::main]
async fn main() {
    env_logger::init();

    let nb_nodes = rand::rng().random_range(4..=10);
    let config = Config { total_nodes: nb_nodes, block_timeout_ms: 1000};

    log::info!("🚀 LANÇEMENT DU RÉSEAU MYRUSTCHAIN (avec {} nœuds)", config.total_nodes);

    let mut engine = ConsensusEngine::new(config).await;
    let mut block_index = 1;

    loop {
        let success = engine.run_next_cycle(block_index).await;

        if success {
            block_index += 1;
        } else {
            log::warn!("🔄 Nouvelle tentative pour le bloc {}", block_index);
        }
    }
}
use myrustchain::consensus::message::ConsensusMessage;
use myrustchain::core::transaction::Transaction;
use myrustchain::consensus::pbft::Node;
use myrustchain::core::block::Block;

#[tokio::test]
async fn test_full_consensus_flow() {
    let mut node = Node::new(0);
    let quorum = 1; // Pour le test, on simplifie le quorum

    let tx = Transaction {
        sender: "Alice".to_string(),
        receiver: "Bob".to_string(),
        amount: 5,
    };
    
    let block = Block::new_block(1, node.blockchain.chain[0].hash.clone(), vec![tx]);

    // Phase 1: PrePrepare -> Prepare
    let msg_preprepare = ConsensusMessage::PrePrepare { block: block.clone(), view: 0 };
    let res_prepare = node.receive_message(msg_preprepare, quorum).unwrap();

    // Phase 2: Prepare -> Commit
    let res_commit = node.receive_message(res_prepare, quorum).unwrap();

    // Phase 3: Commit -> Finalisation
    node.receive_message(res_commit, quorum);

    assert_eq!(node.blockchain.chain.len(), 2);
    assert_eq!(node.blockchain.chain.last().unwrap().hash, block.hash);
}

#[tokio::test]
async fn test_duplicate_vote_rejection() {
    let mut node = Node::new(0);
    let hash = "hash_123".to_string();
    let quorum = 2;

    node.receive_message(ConsensusMessage::Prepare { block_hash: hash.clone(), node_id: 1 }, quorum);
    let res = node.receive_message(ConsensusMessage::Prepare { block_hash: hash.clone(), node_id: 1 }, quorum);

    assert!(res.is_none()); // Un seul vote par ID autorisé
}
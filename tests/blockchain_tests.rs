use blockchain::{Blockchain, Transaction};

#[test]
fn test_blockchain_valide() {
    let mut bc = Blockchain::new_blockchain();
    let txs = vec![Transaction {
        sender: String::from("Alice"),
        receiver: String::from("Bob"),
        amount: 10,
    }];
    bc.add_block(txs);
    assert!(bc.is_chain_valid());
}

#[test]
fn test_fraude_detectee() {
    let mut bc = Blockchain::new_blockchain();
    let txs = vec![Transaction {
        sender: String::from("Alice"),
        receiver: String::from("Bob"),
        amount: 10,
    }];
    bc.add_block(txs);

    bc.chain[1].data[0].amount = 1000; 

    assert!(!bc.is_chain_valid());
}
use crate::consensus::pow::ProofOfWork;
use crate::core::block::Block;
use super::*;

#[test]
fn init_chain() {
    let mut chain = Blockchain::new();
    chain.add_block();
    chain.add_block();

    assert_eq!(true, chain.is_valid());
}

#[test]
fn tamper_block() {
    let mut chain = Blockchain::new();
    chain.add_block();
    chain.add_block();

    chain.chain[1].prev_hash = "fake".to_string();
    assert_eq!(false, chain.is_valid());
}

#[test]
fn valid_tx() {
    let wallet1 = generate_wallet();
    let wallet2 = generate_wallet();

    let mut tx = Transaction {
        from: wallet1.public.to_bytes().to_vec(),
        to: wallet2.public.to_bytes().to_vec(),
        amount: 50,
        signature: vec![],
    };

    tx.signature = sign_transaction(&tx, &wallet1);

    println!("Valid tx: {}", verify_transaction(&tx));
}

#[test]
fn pow_test() {
    let mut block = Block::new(0, "0".to_string());
    let pow = ProofOfWork {
        difficulty: 4
    };

    pow.mine(&mut block);
    assert!(block.hash.starts_with(&"0".repeat(pow.difficulty)));
}

#[test]
fn pow_tamper() {
    let mut block = Block::new(0, "0".to_string());
    block.hash = "fake".to_string();
    
    let pow = ProofOfWork{
        difficulty: 4
    };
    
    assert!(!pow.validate(&block));
}
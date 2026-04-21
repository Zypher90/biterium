use std::hint::black_box;
use sha2::digest::typenum::Prod;
use crate::core::transaction::verify_transaction;
use super::block::Block;
use crate::consensus::pow::ProofOfWork;
pub struct Blockchain {
    pub chain: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Self{
        let genesis = Block::new(0, "0".to_string());

        let blockchain = Blockchain{
            chain: vec![genesis]
        };

        blockchain
    }

    pub fn add_block(&mut self) {
        let prev_block = self.chain.last().unwrap();

        let mut new_block = Block::new(
            prev_block.index+1,
            prev_block.hash.clone()
        );

        let pow = ProofOfWork{
            difficulty: 4
        };
        pow.mine(&mut new_block);

        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool{
        for i in 1..self.chain.len(){
            let current = &self.chain[i];
            let prev = &self.chain[i-1];

            for tx in &current.transactions {
                if !verify_transaction(tx) {
                    return false;
                }
            }
            
            let pow = ProofOfWork{
                difficulty: 4
            };
            if !pow.validate(&current) {
                return false;
            }

            if current.hash != current.calculate_hash() {
                return false;
            }

            if current.prev_hash != prev.hash {
                return false;
            }
        }
        true
    }
}
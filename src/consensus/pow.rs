use crate::core::block::Block;

pub struct ProofOfWork {
    pub difficulty: usize
}

impl ProofOfWork {
    pub fn mine(&self, block: &mut Block) {
        let target = "0".repeat(self.difficulty);
        loop {
            let hash = block.calculate_hash();

            if hash.starts_with(&target) {
                block.hash = hash;
                break;
            }

            block.nonce+=1;
        }
    }

    pub fn validate(&self, block: &Block) -> bool{
        let target = "0".repeat(self.difficulty);
        let hash = block.calculate_hash();

        hash.starts_with(&target) && block.hash == hash
    }
}
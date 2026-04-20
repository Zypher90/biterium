use super::block::Block;
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

        let new_block = Block::new(prev_block.index+1, prev_block.hash.clone());

        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool{
        for i in 1..self.chain.len(){
            let current = &self.chain[i];
            let prev = &self.chain[i-1];

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
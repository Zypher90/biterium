use super::block::Block;

struct Blockchain {
    chain: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Self{
        let genesis = Block::new(0, "0".to_string());

        let blockchain = Blockchain{
            chain: vec![genesis]
        };

        blockchain
    }

    pub fn add_block(block: Block) {

    }
}
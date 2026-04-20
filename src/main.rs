mod core;
use core::blockchain::Blockchain;

fn main() {

}

#[cfg(test)]
mod tests {
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
}
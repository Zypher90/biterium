use crate::core::block::Block;
use crate::core::transaction::Transaction;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum  Message {
    NewTransaction(Transaction),
    NewBlock(Block),
    RequestChain,
    ResponseChain(Vec<Block>)
}
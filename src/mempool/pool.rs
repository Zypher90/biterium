use std::collections::HashMap;
use crate::core::transaction::{verify_transaction, Transaction};

pub struct Mempool{
    pub pool: HashMap<String, Transaction>
}

impl Mempool {
    pub fn add_transaction(&mut self, tx: &Transaction, balances: &HashMap<Vec<u8>, u64>) -> bool {
        if(!verify_transaction(&tx)) {
            return false;
        }
        
        if(!tx.verify_balance(balances)) {
            return false;
        }

        let id = tx.id();

        if self.pool.contains_key(&id) {
            return false;
        }

        self.pool.insert(id, tx.clone());
        true
    }
    
    pub fn get_transactions(&self, limit: usize) -> Vec<Transaction> {
        self.pool
            .values()
            .take(limit)
            .cloned()
            .collect()
    }
    
    pub fn delete_transactions(&mut self, txs: &Vec<Transaction>) {
        for tx in txs {
            let id = tx.id();
            
            self.pool.remove(&id);
        }
    }
}
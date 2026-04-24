use std::collections::HashMap;
use ed25519_dalek::{Keypair, PublicKey, Signer, Verifier, Signature};
use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    pub amount: u64,
    pub signature: Vec<u8>
}

impl Transaction {
    pub fn id(&self) -> String{
        let tx = serde_json::to_vec(&(
                &self.to,
                &self.from,
                self.amount,
                &self.signature
            )).unwrap();

        let mut hasher = Sha256::new();
        hasher.update(tx);
        format!("{:x}", hasher.finalize())
    }

    pub fn verify_balance(
        &self,
        balances: &HashMap<Vec<u8>, u64>
    ) -> bool {
        let balance = balances.get(&self.from.clone()).unwrap_or(&0);

        *balance >= self.amount
    }
}

pub fn sign_transaction(tx: &Transaction, kp: &Keypair) -> Vec<u8> {
    let data = serde_json::to_vec(&(
            &tx.from,
            &tx.to,
            tx.amount
        )).unwrap();

    kp.sign(&data).to_bytes().to_vec()
}

pub fn verify_transaction(tx: &Transaction) -> bool {
    let data = serde_json::to_vec(
        &(
                &tx.from,
                &tx.to,
                tx.amount
        )
    ).unwrap();

    let pubkey = PublicKey::from_bytes(&tx.from).unwrap();
    let signature = Signature::from_bytes(&tx.signature).unwrap();

    pubkey.verify(&data, &signature).is_ok()
}

pub fn balance_transactions(txs: &[Transaction], balances: &mut HashMap<Vec<u8>, u64>) -> bool {
    for tx in txs {
        let balance = balances.get(&tx.from.clone()).unwrap_or(&0);

        if *balance < tx.amount {
            return false;
        }

        *balances.entry(tx.from.clone()).or_insert(0) -= tx.amount;
        *balances.entry(tx.to.clone()).or_insert(0) += tx.amount;
    }

    true
}
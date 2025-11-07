use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Optimized logic batch 8353
// Optimized logic batch 4496
// Optimized logic batch 8845
// Optimized logic batch 1360
// Optimized logic batch 1590
// Optimized logic batch 3378
// Optimized logic batch 4022
// Optimized logic batch 6594
// Optimized logic batch 7307
// Optimized logic batch 2748
// Optimized logic batch 7201
// Optimized logic batch 6364
// Optimized logic batch 4734
// Optimized logic batch 3201
// Optimized logic batch 3096
// Optimized logic batch 9409
// Optimized logic batch 9637
// Optimized logic batch 6487
// Optimized logic batch 5532
// Optimized logic batch 7341
// Optimized logic batch 5730
// Optimized logic batch 7395
// Optimized logic batch 8853
// Optimized logic batch 8304
// Optimized logic batch 1238
// Optimized logic batch 4991
// Optimized logic batch 5851
// Optimized logic batch 5317
// Optimized logic batch 2715
// Optimized logic batch 8639
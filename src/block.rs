use std::fmt::{self, Debug, Formatter};

use crate::difficulty_bytes_as_u128;
use crate::u128_bytes;
use crate::u32_bytes;
use crate::u64_bytes;
use crate::Hash;
use crate::Hashable;
use crate::Transaction;

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_hash_block: Hash,
    pub nonce: u64,
    pub difficulty: u128,
    pub transactions: Vec<Transaction>,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block [{}]: {} at: {} nonce: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.nonce,
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_hash_block: Hash,
        difficulty: u128,
        transaction: Vec<Transaction>,
    ) -> Self {
        Self {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_hash_block,
            nonce: 0,
            difficulty,
            transactions: transaction,
        }
    }
    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_hash_block);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(&u128_bytes(&self.difficulty));
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>(),
        );
        return bytes;
    }
}

pub fn check_difficulty(hash: &Hash, diffculty: u128) -> bool {
    return diffculty > difficulty_bytes_as_u128(&hash);
}

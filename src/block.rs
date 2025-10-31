use std::fmt::{self, Debug, Formatter};

use crate::difficulty_bytes_as_u128;
use crate::u128_bytes;
use crate::u32_bytes;
use crate::u64_bytes;
use crate::BlockHash;
use crate::Hashable;

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_hash_block: BlockHash,
    pub nonce: u64,
    pub payload: String,
    difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block [{}]: {} at: {} with : {} nonce: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.payload,
            &self.nonce
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_hash_block: BlockHash,
        nonce: u64,
        payload: String,
        difficulty: u128,
    ) -> Self {
        Self {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_hash_block,
            nonce,
            payload,
            difficulty,
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
        bytes.extend(self.payload.as_bytes());
        bytes.extend(&u128_bytes(&self.difficulty));
        return bytes;
    }
}

pub fn check_difficulty(hash: &BlockHash, diffculty: u128) -> bool {
    return diffculty > difficulty_bytes_as_u128(&hash);
}

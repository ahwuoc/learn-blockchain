use core::hash;
use std::{collections::HashSet, fs::FileTimes, vec};

use super::*;

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPrevHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficienInputValue,
    InvalidCoinbaseTransaction,
}

pub struct BlockChain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
}

impl BlockChain {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }
    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationErr> {
        let i = self.blocks.len();
        if block.index != i as u32 {
            return Err(BlockValidationErr::MismatchedIndex);
        }
        if !block::check_difficulty(&block.hash, block.difficulty) {
            println!("Difficulty check failed");
            return Err(BlockValidationErr::InvalidHash);
        } else if i != 0 {
            let prev_block = &self.blocks[i - 1];
            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationErr::AchronologicalTimestamp);
            } else if block.prev_hash_block != prev_block.hash {
                return Err(BlockValidationErr::MismatchedPrevHash);
            }
        } else {
            //Genesis Block
            if block.prev_hash_block != vec![0; 32] {
                return Err(BlockValidationErr::InvalidGenesisBlockFormat);
            }
        }
        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }
            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;
            for transaction in transactions {
                let input_hashes = transaction.input_hashes();
                if !(&input_hashes - &self.unspent_outputs).is_empty()
                    || !(&input_hashes & &block_spent).is_empty()
                {
                    return Err(BlockValidationErr::InvalidInput);
                }
                let input_value = transaction.input_values();
                let output_value = transaction.output_values();

                if output_value > input_value {
                    return Err(BlockValidationErr::InsufficienInputValue);
                }

                let fee = input_value - output_value;
                total_fee += fee;
                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }
            if coinbase.output_values() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(coinbase.output_hashes());
            }
            self.unspent_outputs
                .retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }
        self.blocks.push(block);
        Ok(())
    }
}

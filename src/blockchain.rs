use super::*;
pub struct BlockChain {
    pub blocks: Vec<Block>,
}
impl BlockChain {
    pub fn verify(&self) -> bool {
        for (i, b) in self.blocks.iter().enumerate() {
            if b.index != i as u32 {
                println!("Index mismatch: {:?}, {:?}", &b.index, &i);
                return false;
            }
            if !block::check_difficulty(&b.hash, b.difficulty) {
                println!("Difficulty check failed");
                return false;
            }
            if b.hash != b.hash() {
                println!("Block hash doesn't match content");
                return false;
            } else if i != 0 {
                //Not genesis block
                let prev_block = &self.blocks[i - 1];
                if b.timestamp <= prev_block.timestamp {
                    println!("Time went backwards");
                    return false;
                } else if b.prev_hash_block != prev_block.hash {
                    println!("Hash mismatch");
                    return false;
                }
            } else {
                // genesis block
                if b.prev_hash_block != vec![0; 32] {
                    println!("Genesis block prev_block_hash invalid");
                    return false;
                }
            }
        }
        true
    }
}

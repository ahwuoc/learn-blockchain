use blockchainlib::{current_timestamp, Block, BlockChain};

fn main() {
    let difficulty = 0x00000FFFFFFFFFFFFFFFFFFFFFFFFFFF;
    let index = 0;
    let mut block = Block::new(
        index,
        current_timestamp(),
        vec![0; 32],
        0,
        "Genesis block".to_owned(),
        difficulty,
    );
    block.mine();
    println!("Mined genesis block{:?}", &block);
    let mut last_hash = block.hash.clone();

    let mut blockchain = BlockChain {
        blocks: vec![block],
    };

    // ===========Verify==============

    for i in 1..=10 {
        let mut block = Block::new(
            i,
            current_timestamp(),
            last_hash,
            0,
            "Block".to_owned(),
            difficulty,
        );
        block.mine();
        println!("Mined genesis block{:?}", &block);
        last_hash = block.hash.clone();
        blockchain.blocks.push(block);
        println!("Blockchain valid: {}", blockchain.verify());
    }
    // blockchain.blocks[2].prev_hash_block[18] = 8;
    println!("Blockchain valid: {}", blockchain.verify());
}

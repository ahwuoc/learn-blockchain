use blockchainlib::{current_timestamp, Block, Hashable};

fn main() {
    let difficulty = 0x000000FFFFFFFFFFFFFFFFFFFFFFFFFF;
    let index = 13;
    let mut block = Block::new(
        index,
        current_timestamp(),
        vec![0; 32],
        0,
        "Genesis block".to_owned(),
        difficulty,
    );
    block.hash = block.hash();

    println!("{:?}", &block);
    block.mine();
    println!("{:?}", &block);
}

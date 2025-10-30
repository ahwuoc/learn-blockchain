use blockchainlib::{current_timestamp, Block, Hashable};
fn main() {
    let mut block = Block::new(
        13,
        current_timestamp(),
        vec![0; 32],
        0,
        "Genesis block".to_owned(),
    );
    println!("{:?}", &block);

    let h = block.hash();
    println!("{:?}", &h);
    block.hash = h; //->owner
}

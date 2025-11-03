use blockchainlib::*;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(
        0,
        current_timestamp(),
        vec![0; 32],
        difficulty,
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                OutPut {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                OutPut {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                },
            ],
        }],
    );

    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = BlockChain::new();

    blockchain
        .update_with_block(genesis_block)
        .expect("Failed to add genesis block");

    let mut block = Block::new(
        1,
        current_timestamp(),
        last_hash,
        difficulty,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![OutPut {
                    to_addr: "Chris".to_owned(),
                    value: 536,
                }],
            },
            Transaction {
                inputs: vec![blockchain.blocks[0].transactions[0].outputs[0].clone()],
                outputs: vec![
                    OutPut {
                        to_addr: "Alice".to_owned(),
                        value: 360,
                    },
                    OutPut {
                        to_addr: "Bob".to_owned(),
                        value: 12,
                    },
                ],
            },
        ],
    );

    block.mine();

    println!("Mined block {:?}", &block);

    last_hash = block.hash.clone();

    blockchain
        .update_with_block(block)
        .expect("Failed to add block");
}

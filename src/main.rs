use ekolib::*;

fn main() {
    let mut nonce = rehash(&vec![1; 32]);
    // difficulty = most significant 2 bytes of 32-bit integer have to be 0 to be considered "mined"]
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut block = Block::new(0, now(), vec![0; 32], nonce, "Genesis block".to_owned(), difficulty);
    block.mine();

    println!("New block:\n\t{:?}", &block);
    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    for i in 1..=10 {
        nonce = rehash(&last_hash);
        // difficulty = most significant 2 bytes of 32-bit integer have to be 0 to be considered "mined"]
        block = Block::new(i, now(), last_hash, nonce, "Mined block".to_owned(), difficulty);
        block.mine();
        last_hash = block.hash.clone();
        println!("New block:\n\t{:?}", &block);
        blockchain.blocks.push(block);
        println!("Verify: {}", blockchain.verify());
    }

    // blockchain.blocks[3].index = 5;
    // println!("Verify: {}", blockchain.verify());
    // blockchain.blocks[3].hash[31] += 1;
    // println!("Verify: {}", blockchain.verify());
    // blockchain.blocks[3].payload = "Difficulty fail".to_owned();
    // println!("Verify: {}", blockchain.verify());
    blockchain.blocks[3].prev_block_hash[31] += 1;
    println!("Verify: {}", blockchain.verify());
}

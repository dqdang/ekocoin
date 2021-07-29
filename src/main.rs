use ekolib::*;

fn main() {
    let nonce = rehash(&vec![1; 32]);
    // difficulty = most significant 2 bytes of 32-bit integer have to be 0 to be considered "mined"
    let mut block = Block::new(1, now(), vec![0; 32], nonce, "Genesis block".to_owned(),
                    0x0000ffffffffffffffffffffffffffff);
    block.hash = block.hash();
    println!("New block:\n\t{:?}", &block);
    block.mine();
    println!("New block:\n\t{:?}", &block);
}

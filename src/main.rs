use ekolib::*;

fn main() {
    let mut block = Block::new(1, now(), vec![0; 32], 5);
    block.hash = block.hash();
    println!("New block:\n\t{:?}", &block);
}

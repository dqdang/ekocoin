use ekolib::*;

fn main() {
    let mut nonce = rehash(&vec![1; 32]);
    // difficulty = most significant 2 bytes of 32-bit integer have to be 0 to be considered "mined"]
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut g_block = Block::new(0, now(), vec![0; 32], nonce, vec![
        Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 15,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                }
            ]
        }
    ], "Genesis block".to_owned(), difficulty);
    g_block.mine();

    println!("New block:\n\t{:?}", &g_block);
    let mut last_hash = g_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(g_block).expect("Failed to add the genesis block");

    nonce = rehash(&last_hash);
    let mut block = Block::new(1, now(), last_hash, nonce, vec![
        Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Derek".to_owned(),
                    value: 123,
                },
            ],
        },
        Transaction {
            inputs: vec![
                blockchain.blocks[0].transactions[0].outputs[1].clone(),
            ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 6,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 1,
                },
            ],
        },
    ], "New block".to_owned(), difficulty);
    block.mine();
    last_hash = block.hash.clone();
    println!("New block:\n\t{:?}\nLast hash:\n\t{:?}", &block, &hex::encode(&last_hash));
    blockchain.update_with_block(block).expect("Failed to add block");
}

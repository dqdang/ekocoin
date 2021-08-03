use super::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash, // doesn't match difficulty, previous transactions
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::<Hash>::new(),
        }
    }

    // pub fn verify(&self) -> bool {
    //     for (i, block) in self.blocks.iter().enumerate() {
    //         if block.index != i as u32 {
    //             println!("Index mismatch: {} != {}", &block.index, &i);
    //             return false;
    //         }
    //         else if !block::check_difficulty(&block.hash(), block.difficulty) {
    //             println!("Difficulty failed");
    //             return false;
    //         }
    //         else if i != 0 {
    //             // println!("Not genesis block");
    //             let previous_block = &self.blocks[i - 1];
    //             if block.timestamp <= previous_block.timestamp {
    //                 println!("Time did not increase.");
    //                 return false;
    //             }
    //             else if previous_block.hash != block.prev_block_hash {
    //                 println!("Previous block hash mismatch: {} != {}",
    //                         hex::encode(&previous_block.hash), hex::encode(&block.prev_block_hash));
    //                 return false;
    //             }
    //         }
    //         else {
    //             // Genesis block
    //             if block.prev_block_hash != vec![0; 32] {
    //                 println!("Genesis block previous block invalid!");
    //                 return false;
    //             }
    //         }
    //     }
    //     return true;
    // }

    pub fn update_with_block (&mut self, block: Block) -> Result<(), BlockValidationErr> {
        let i = self.blocks.len();

        if block.index != i as u32 {
            return Err(BlockValidationErr::MismatchedIndex);
        }
        else if !block::check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValidationErr::InvalidHash);
        }
        else if i != 0 {
            // Not genesis block
            let prev_block = &self.blocks[i - 1];
            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationErr::AchronologicalTimestamp);
            }
            else if block.prev_block_hash != prev_block.hash {
                return Err(BlockValidationErr::MismatchedPreviousHash);
            }
        }
        else {
            // Genesis block
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidationErr::InvalidGenesisBlockFormat);
            }
        }

        // if first element of transactions exists
        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }

            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();

                if // there exists an input not in unspent outputs or input not in set of spent outputs
                    !(&input_hashes - &self.unspent_outputs).is_empty() ||
                    !(&input_hashes & &block_spent).is_empty()
                {
                    return Err(BlockValidationErr::InvalidInput);
                }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                if output_value > input_value { // make sure we don't output 7000 coins from 1 coin
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = input_value - output_value;

                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }

            if coinbase.output_value() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }
            else {
                block_created.extend(coinbase.output_hashes());
            }

            // remove all the spent outputs, keep the ones that are not spent
            self.unspent_outputs.retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }

        self.blocks.push(block);

        return Ok(())
    }
}

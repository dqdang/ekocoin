use super::*;
use std::fmt::{self, Debug, Formatter};

// #[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: Hash,
    pub transactions: Vec<Transaction>,
    pub payload: String,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block[{}]: {} / {} at: {} with: nonce: {}, Transaction: {}",
            &self.index,
            self.payload,
            &hex::encode(&self.hash),
            &self.timestamp,
            &hex::encode(&self.nonce),
            &self.transactions.len(),
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Hash,
        nonce: Hash,
        transactions: Vec<Transaction>,
        payload: String,
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce,
            transactions,
            payload,
            difficulty,
        }
    }

    pub fn mine(&mut self) {
        for _nonce_attempt in 0..(u64::max_value()) {
            self.nonce = rehash(&self.hash());
            let hash = self.hash();
            // println!("{:?}", &hex::encode(&hash));
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&self.nonce);
        bytes.extend(
            self.transactions.iter()
            .flat_map(|transaction| transaction.bytes())
            .collect::<Vec<u8>>(),
        );
        bytes.extend(self.payload.as_bytes());
        bytes.extend(&u128_bytes(&self.difficulty));

        return bytes;
    }
}

pub fn check_difficulty(hash: &Hash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}

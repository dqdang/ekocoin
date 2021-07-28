use blake2::{Blake2s, Digest};

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> Vec<u8> {
        let mut hasher = Blake2s::new();
        hasher.update(&self.bytes());
        let res = hasher.finalize();
        return res.to_vec();
    }
}

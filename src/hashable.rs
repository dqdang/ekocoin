use super::*;

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> Vec<u8> {
        return rehash(&self.bytes());
    }
}

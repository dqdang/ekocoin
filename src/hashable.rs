use super::*;

pub trait Hashable {
    fn bytes(&self) -> Hash;

    fn hash(&self) -> Hash {
        return rehash(&self.bytes());
    }
}

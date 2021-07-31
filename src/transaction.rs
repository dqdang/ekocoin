use super::*;

pub struct Output {
    pub to_addr: Address,
    pub value: u64,
}

impl Hashable for Output {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&u64_bytes(&self.value));

        return bytes;
    }
}

pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(
            self.inputs.iter()
            .flat_map(|input| -> input.bytes())
            .collect::<<Vec><u8>>();
        )
        bytes.extend(self.outputs.as_bytes());

        return bytes;
    }
}

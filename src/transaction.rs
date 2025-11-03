use crate::{u64_bytes, Address, Hash, Hashable};
use std::collections::HashSet;

#[derive(Clone)]
pub struct OutPut {
    pub to_addr: Address,
    pub value: u64,
}
pub struct Transaction {
    pub inputs: Vec<OutPut>,
    pub outputs: Vec<OutPut>,
}

impl Hashable for OutPut {
    fn bytes(&self) -> Hash {
        let mut bytes = vec![];
        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&u64_bytes(&self.value));
        return bytes;
    }
}

impl Transaction {
    pub fn input_values(&self) -> u64 {
        return self.inputs.iter().map(|i| i.value).sum();
    }
    pub fn output_values(&self) -> u64 {
        return self.outputs.iter().map(|o| o.value).sum();
    }
    pub fn input_hashes(&self) -> HashSet<Hash> {
        self.inputs
            .iter()
            .map(|input| input.hash())
            .collect::<HashSet<Hash>>()
    }
    pub fn output_hashes(&self) -> HashSet<Hash> {
        self.outputs
            .iter()
            .map(|ouput| ouput.hash())
            .collect::<HashSet<Hash>>()
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Hash {
        let mut bytes = vec![];

        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes())
                .collect::<Hash>(),
        );
        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|output| output.bytes())
                .collect::<Hash>(),
        );

        bytes
    }
}

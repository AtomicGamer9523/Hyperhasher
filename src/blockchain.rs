//AUTHOR: "AtomicGamer9523"@github.com
//LICENSE: "MIT"
//FORMAT: "RUST"
use crate::consts::INITIAL_BLOCK_PREVIOS_HASH;
use std::fmt::{
    Formatter,
    Display,
    Result
};
pub const IBD: &'static str = "INITIAL.BLOCK.DATA";





#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
pub struct HashString {
    strng: [u8;64]
}
impl HashString {
    pub(crate) fn from_generic_array(strng: [u8;64]) -> HashString { HashString {strng}}
    pub fn to_str(&self) -> String {
        hex::encode(self.strng)
    }
}
impl Display for HashString {
    fn fmt(&self, f:&mut Formatter ) -> Result {
        write!(
            f,
            "{}",
            hex::encode(self.strng)
        )
    }
}
impl Default for HashString {
    fn default() -> Self {
        Self {
            strng: INITIAL_BLOCK_PREVIOS_HASH
        }
    }
}





#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Block <T: Copy + Display> {
    pub previos_hash: HashString,
    pub data: T
}
impl<T: Copy + Display> Display for Block<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}+{}",
            self.previos_hash,
            self.data
        )
    }
}
impl<T: Copy + Display> Block<T> {
    pub fn new(data: T, chain: &Chain) -> Block<T> {
        Block {
            previos_hash: chain.hash,
            data: data
        }
    }
    pub(crate) fn initial(data: T) -> Block<T> {
        Block {
            previos_hash: HashString::default(),
            data
        }
    }
}





#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Chain {
    pub hash: HashString
}
impl Chain {
    pub fn new<T: Copy + Display>(initial_block_data: T) -> Self {
        Self {
            hash: crate::hash!(Block::initial(initial_block_data))
        }
    }
    pub fn push<T: Copy + Display>(&mut self, block: Block<T>) -> HashString {
        self.hash = crate::hash!(block);
        self.hash
    }
}
impl Display for Chain {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            self.hash
        )
    }
}
//AUTHOR: "AtomicGamer9523"@github.com
//LICENSE: "MIT"
//FORMAT: "RUST"
use std::fmt::{
    Formatter,
    Display,
    Result
};
pub use blake3;
pub const IBD: &'static str = "INITIAL.BLOCK.DATA";








#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct HashString([u8;32]);
impl HashString {
    pub fn new(bytes: [u8;32]) -> Self {
        Self(bytes)
    }
    fn to_blakehash(&self) -> blake3::Hash {
        blake3::Hash::from(self.0)
    }
}
impl From<[u8;32]> for HashString {
    fn from(bytes: [u8;32]) -> Self {
        Self(bytes)
    }
}
impl From<&[u8;32]> for HashString {
    fn from(bytes: &[u8;32]) -> Self {
        Self(*bytes)
    }
}
impl From<blake3::Hash> for HashString {
    fn from(hash: blake3::Hash) -> Self {
        HashString::from(hash.as_bytes())
    }
}
impl From<HashString> for [u8;32] {
    fn from(hash: HashString) -> Self {
        hash.0
    }
}
impl Display for HashString {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let hex = self.to_blakehash().to_hex();
        let hex: &str = hex.as_str();
        f.write_str(hex)
    }
}






#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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
            previos_hash: HashString::from(crate::consts::INITIAL_BLOCK_PREVIOS_HASH),
            data
        }
    }
}





#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
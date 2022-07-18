//AUTHOR: "AtomicGamer9523"@github.com
//LICENSE: "MIT"
//FORMAT: "RUST"
use std::fmt::{
    Formatter,
    Display,
    Result
};
pub use blake3::Hash;
pub const IBD: &'static str = "INITIAL.BLOCK.DATA";




#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Block <T: Copy + Display> {
    pub previos_hash: Hash,
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
            previos_hash: Hash::from(crate::consts::INITIAL_BLOCK_PREVIOS_HASH),
            data
        }
    }
}





#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Chain {
    pub hash: Hash
}
impl Chain {
    pub fn new<T: Copy + Display>(initial_block_data: T) -> Self {
        Self {
            hash: crate::hash!(Block::initial(initial_block_data))
        }
    }
    pub fn push<T: Copy + Display>(&mut self, block: Block<T>) -> Hash {
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
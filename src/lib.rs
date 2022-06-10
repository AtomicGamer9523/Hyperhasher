//AUTHOR: "AtomicGamer9523"@github.com
//LICENSE: "Creative Commons CC0 1.0"
//FORMAT: "RUST"
use std::fmt::{Display,Formatter,Result};
#[allow(unused_imports)]
use chrono::offset::Local;



pub const INITIAL_BLOCK_PREVIOS_HASH: [u8;64] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
#[allow(dead_code)]
const EXPECTED_HASH_RESULT: [u8;64] = [120,106,2,247,66,1,89,3,198,198,253,133,37,82,210,114,145,47,71,64,225,88,71,97,138,134,226,23,247,31,84,25,210,94,16,49,175,238,88,83,19,137,100,68,147,78,176,75,144,58,104,91,20,72,183,85,213,111,112,26,254,155,226,206];
#[allow(dead_code)]
const EXPECTED_SIMPLE_CHAIN_RESULT: [u8;64] = [39,228,0,67,184,43,222,114,159,209,110,32,130,235,177,114,251,112,57,148,120,58,48,235,232,137,19,122,142,71,86,3,18,59,114,94,101,249,157,200,146,202,20,186,165,53,81,90,231,185,60,16,73,47,228,248,114,131,143,190,104,55,249,207];
pub const INITIAL_BLOCK_DATA: &'static str = "INITIAL.BLOCK.DATA";



#[macro_export]
macro_rules! info {($e:expr)=>{{print!("\x1b[38;5;236m{} \x1b[38;5;92m\x1b[1mINFO\x1b[0m \x1b[38;5;0m[\x1b[38;5;24m{}\x1b[38;5;0m]\x1b[0m {}",Local::now().format("%Y/%m/%d %H:%M:%S,%6f"),env!("CARGO_PKG_NAME"),$e);}};}
#[macro_export]
macro_rules! debug {($e:expr)=>{{print!("\x1b[38;5;236m{} \x1b[38;5;26m\x1b[1mDEBUG\x1b[0m \x1b[38;5;0m[\x1b[38;5;24m{}\x1b[38;5;0m]\x1b[0m {}",Local::now().format("%Y/%m/%d %H:%M:%S,%6f"),env!("CARGO_PKG_NAME"),$e);}};}
#[macro_export]
macro_rules! warn {($e:expr)=>{{print!("\x1b[38;5;236m{} \x1b[38;5;166m\x1b[1mWARN\x1b[0m \x1b[38;5;0m[\x1b[38;5;24m{}\x1b[38;5;0m]\x1b[0m {}",Local::now().format("%Y/%m/%d %H:%M:%S,%6f"),env!("CARGO_PKG_NAME"),$e);}};}
#[macro_export]
macro_rules! error {($e:expr)=>{{print!("\x1b[38;5;236m{} \x1b[38;5;1m\x1b[1mERROR\x1b[0m \x1b[38;5;0m[\x1b[38;5;24m{}\x1b[38;5;0m]\x1b[0m {}",Local::now().format("%Y/%m/%d %H:%M:%S,%6f"),env!("CARGO_PKG_NAME"),$e);std::process::exit(1);}};}








#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
pub struct HashString {
    strng: [u8;64]
}
impl HashString {
    pub fn from_generic_array(generic_array: [u8;64]) -> HashString {
        HashString {
            strng: generic_array
        }
    }
}
impl Display for HashString {
    fn fmt(&self, f:&mut Formatter ) -> Result {
        write!(
            f,
            "{:?}",
            self.strng
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










#[macro_export]
macro_rules! hash {($e:expr)=>{{use blake2::{Digest,Blake2b512};use std::convert::TryInto;let mut hasher=Blake2b512::new();hasher.update(format!("{}",$e).as_bytes());let hashres:[u8;64]=hasher.finalize().as_slice().try_into().expect("Wrong Length");HashString::from_generic_array(hashres)}};}





#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Block {
    pub previos_hash: HashString,
    pub data: String
}
impl Display for Block {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}+{}",
            self.previos_hash,
            self.data
        )
    }
}
impl Block {
    pub fn new(data: String, chain: &Chain) -> Block {
        Block {
            previos_hash: chain.hash,
            data: data
        }
    }
    pub(crate) fn initial(data: String) -> Block {
        Block {
            previos_hash: HashString::default(),
            data
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Chain {
    pub hash: HashString
}
impl Chain {
    pub fn new(initial_block_data: String) -> Self {
        Self {
            hash: hash!(Block::initial(initial_block_data))
        }
    }
    pub fn push(&mut self, block: Block) {
        self.hash = hash!(block);
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



#[test]
fn blake2_hash_test() {
    let hash = hash!("");
    assert_eq!(hash, HashString::from_generic_array(EXPECTED_HASH_RESULT));
}

#[test]
fn simple_chain_test() {
    let mut chain = Chain::new(String::from(""));
    let block = Block::new(String::from("test"),&chain);
    chain.push(block);
    assert_eq!(chain.hash, HashString::from_generic_array(EXPECTED_SIMPLE_CHAIN_RESULT));
}
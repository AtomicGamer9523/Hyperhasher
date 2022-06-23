//AUTHOR: "AtomicGamer9523"@github.com
//LICENSE: "MIT"
//FORMAT: "RUST"
//! A light-weight Blockchain library using [BLAKE2][1] for hashing.
//!
//! # Usage:
//!
//! ```rust
//! use hyperhasher::{Block, Chain};
//!
//! fn main(){
//!     // creating a Chain
//!     let mut chain = Chain::new(
//!         //data for initial block
//!         String::from("")
//!     );
//!     let block = Block::new(String::from("test"), &chain);
//!     chain.push(block);
//! }
//! ```
//!
//! Also see [RustCrypto/hashes](https://github.com/RustCrypto/hashes) readme.
//!
//! ## Variable output size
//!
//! This implementation supports run and compile time variable sizes.
//!
//! Run time variable output example:
//! ```rust
//! use blake2::Blake2bVar;
//! use blake2::digest::{Update, VariableOutput};
//! use hex_literal::hex;
//!
//! let mut hasher = Blake2bVar::new(10).unwrap();
//! hasher.update(b"my_input");
//! let mut buf = [0u8; 10];
//! hasher.finalize_variable(&mut buf).unwrap();
//! assert_eq!(buf, hex!("2cc55c84e416924e6400"));
//! ```
//!
//! Compile time variable output example:
//! ```rust
//! use blake2::{Blake2b, Digest, digest::consts::U10};
//! use hex_literal::hex;
//!
//! type Blake2b80 = Blake2b<U10>;
//!
//! let mut hasher = Blake2b80::new();
//! hasher.update(b"my_input");
//! let res = hasher.finalize();
//! assert_eq!(res[..], hex!("2cc55c84e416924e6400")[..]);
//! ```
//!
//! # Acknowledgment
//! Based on the [blake2-rfc][2] crate.
//!
//! [1]: https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE2
//! [2]: https://github.com/cesarb/blake2-rfc
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg"
)]







use std::fmt::{
    Formatter,
    Display,
    Result
};
mod consts;
#[allow(unused_imports)]
use consts::{
    EXPECTED_SIMPLE_CHAIN_RESULT,
    INITIAL_BLOCK_PREVIOS_HASH,
    EXPECTED_HASH_RESULT
};
pub use hex;
pub use chrono;
pub use blake2;



pub const IBD: &'static str = "INITIAL.BLOCK.DATA";



#[macro_export]
macro_rules! info {
    ( $e: expr ) => {{
        print!(
            "\x1b[38;5;236m{} \x1b[38;5;92m\x1b[1mINF\x1b[0m \x1b[38;5;0m[\x1b[38;5;24m{}\x1b[38;5;0m]\x1b[0m {}",
            chrono::offset::Local::now().format("%Y/%m/%d %H:%M:%S,%6f"),
            env!("CARGO_PKG_NAME"),
            $e
        );
    }};
}
#[macro_export]
macro_rules! debug {
    ( $e: expr ) => {{
        print!(
            "\x1b[38;5;236m{} \x1b[38;5;26m\x1b[1mDBG\x1b[0m \x1b[38;5;0m[\x1b[38;5;24m{}\x1b[38;5;0m]\x1b[0m {}",
            chrono::offset::Local::now().format("%Y/%m/%d %H:%M:%S,%6f"),
            env!("CARGO_PKG_NAME"),
            $e
        );
    }};
}
#[macro_export]
macro_rules! warn {
    ( $e: expr ) => {{
        print!(
            "\x1b[38;5;236m{} \x1b[38;5;166m\x1b[1mWRN\x1b[0m \x1b[38;5;0m[\x1b[38;5;24m{}\x1b[38;5;0m]\x1b[0m {}",
            chrono::offset::Local::now().format("%Y/%m/%d %H:%M:%S,%6f"),
            env!("CARGO_PKG_NAME"),
            $e
        );
    }};
}
#[macro_export]
macro_rules! error {
    ( $e: expr ) => {{
        print!(
            "\x1b[38;5;236m{} \x1b[38;5;1m\x1b[1mERR\x1b[0m \x1b[38;5;0m[\x1b[38;5;24m{}\x1b[38;5;0m]\x1b[0m {}",
            chrono::offset::Local::now().format("%Y/%m/%d %H:%M:%S,%6f"),
            env!("CARGO_PKG_NAME"),
            $e
        );
        std::process::exit(1);
    }};
}





#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
pub struct HashString {
    strng: [u8;64]
}
impl HashString {
    pub fn from_generic_array(strng: [u8;64]) -> HashString { HashString {strng}}
    pub fn to_str(&self) -> String {
        hex::encode(self.strng)
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
macro_rules! hash {
    ( $e: expr ) => {{
        use blake2::{Digest, Blake2b512};
        use std::convert::TryInto;
        let mut hasher = Blake2b512::new();
        hasher.update(
            format!(
                "{}",
                $e
            )
            .as_bytes()
        );
        let hashres: [u8;64] = hasher.finalize()
        .as_slice().try_into().expect("Wrong Length");
        HashString::from_generic_array(hashres)
    }};
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
            hash: hash!(Block::initial(initial_block_data))
        }
    }
    pub fn push<T: Copy + Display>(&mut self, block: Block<T>) -> HashString {
        self.hash = hash!(block);
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



#[test]
fn blake2_hash_test() {
    let hash = hash!("");
    assert_eq!(hash, HashString::from_generic_array(EXPECTED_HASH_RESULT));
}

#[test]
fn simple_chain_test() {
    let mut chain = Chain::new(IBD);
    let block = Block::new("test",&chain);
    chain.push(block);
    assert_eq!(chain.hash, HashString::from_generic_array(EXPECTED_SIMPLE_CHAIN_RESULT));
}
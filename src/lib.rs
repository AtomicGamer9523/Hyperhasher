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
//! let mut chain = Chain::new(IBD);
//! let block = Block::new("test", &chain);
//! chain.push(block);
//! assert_eq!(chain.hash, HashString::from_generic_array(EXPECTED_SIMPLE_CHAIN_RESULT));
//! }
//! ```
//!
//! Also see [RustCrypto/hashes](https://github.com/RustCrypto/hashes) readme.
//!
//! [1]: https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE2
//! [2]: https://github.com/cesarb/blake2-rfc
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg"
)]





mod consts;
#[allow(unused_imports)]
use consts::{
    EXPECTED_SIMPLE_CHAIN_RESULT,
    INITIAL_BLOCK_PREVIOS_HASH,
    EXPECTED_HASH_RESULT
};
mod blockchain;
pub use blockchain::{
    HashString,
    Block,
    Chain,
    IBD
};
pub use hex;
pub use chrono;
pub use blake2;





#[test]
fn blake2_hash_test() {
    let hash = hash!("");
    assert_eq!(hash, HashString::from_generic_array(EXPECTED_HASH_RESULT));
}

#[test]
fn simple_chain_test() {
    let mut chain = Chain::new(IBD);
    let block = Block::new("test", &chain);
    chain.push(block);
    assert_eq!(chain.hash, HashString::from_generic_array(EXPECTED_SIMPLE_CHAIN_RESULT));
}
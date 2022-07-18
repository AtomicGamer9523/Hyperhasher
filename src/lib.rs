//AUTHOR: "AtomicGamer9523"@github.com
//LICENSE: "MIT"
//FORMAT: "RUST"
//! A light-weight Blockchain library using [BLAKE2][1] for hashing.
//!
//! # Usage:
//!
//! ```rust
//! use hyperhasher::{
//!     HashString,
//!     Block,
//!     Chain,
//!     IBD
//! };
//!
//! fn main(){
//!     //Craeting A New Chain
//!     let mut chain = Chain::new(IBD);
//!     let block = Block::new("test", &chain);
//!     chain.push(block);
//!     println!("{}", chain.hash);
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
mod macros;
mod consts;
mod blockchain;



#[allow(unused_imports)]
pub use blockchain::{
    Block,
    Chain,
    IBD
};
pub use blake3;
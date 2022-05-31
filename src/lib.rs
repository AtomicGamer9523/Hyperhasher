/*!
**HyperHasher** is A light-weight BlockChain Library.

## Usage Example
```rust
use hyperhasher::{Systems, Block, Chain, config}

fn main() {

    let mut systems = Systems::new();
    
    let conf = config("manifest");
    let mut chain = Chain::new(conf);
    systems.add(Box::new(chain));


    ctrlc::set_handler(move ||{
        chain.clean();
        std::process::exit(0);
    }).unwrap();
    systems.run();
}
*/
mod systems;pub use systems::{System, Systems};
mod config;pub use config::{Config, config};
mod blocks;pub use blocks::{Block, Chain};
pub mod macros;
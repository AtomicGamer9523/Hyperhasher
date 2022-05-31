# Hyper-Hasher

## A Simple BlockChain Library for `rust` and `js`

### Examples

```js
const {Block, Chain} = require('hyperhasher');

let chain = new Chain();

let block = new Block("DATA_FOR_THE_BLOCK",chain);

chain.push(block);

console.log("Chain:", chain);
console.log("Block:", block);
```

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
```

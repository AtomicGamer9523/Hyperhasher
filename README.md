<!-- AUTHOR: "AtomicGamer9523"@github.com -->
<!-- LICENSE: "MIT" -->
<!-- FORMAT: "README" -->

# Hyper-Hasher

## A Simple BlockChain Library for `rust` and `js`

### Examples

```js
const {Block, Chain} = require('hyperhasher');

let chain = new Chain();

let block = new Block("DATA_FOR_THE_BLOCK",chain);

chain.push(block);
```

```rust
use hyperhasher::{Block, Chain};

fn main() {
    let mut chain = Chain::new();
    let block = Block::new("DATA_FOR_THE_BLOCK",&chain);
    chain.push(block);
}
```

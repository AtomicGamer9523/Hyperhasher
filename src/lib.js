//AUTHOR: "AtomicGamer9523"@github.com
//LICENSE: "MIT"
//FORMAT: "JS"
const blake2 = require('./blake2');

class Block {
    previos_hash;
    data;
    constructor(data, chain){
        this.previos_hash = chain.hash;
        this.data = data;
    }
    static _initial(data){
        return new Block(data,{hash:0})
    }
}

class Chain {
    hash;
    constructor(initial_block_data){
        this.hash = blake2(Block._initial(initial_block_data));
    }
    push(block){
        this.hash = blake2(block)
    }
}


exports = {
    Block: Block,
    Chain: Chain
}
module.exports = {
    Block: Block,
    Chain: Chain
}
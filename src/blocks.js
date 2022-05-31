const hasher = require('./macros');
const fs = require('fs');

class Block {
    previos_hash;
    data;
    constructor(data, chain){
        this.data = data;
        this.previos_hash = chain.hash
    }
}

class Chain {
    hash;
    constructor(){
        this.hash = hasher(new Block("INITIAL_BLOCK",{hash:0}))
    }

    push(block){
        let hash = hasher(block);
        this.hash = hash;
        return hash;
    }

    clean(){
        fs.writeFileSync(__dirname+"hash",this.hash,{encoding:"utf-8"});
    }
}

module.exports = {
    Block: Block,
    Chain: Chain
}
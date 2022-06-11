//AUTHOR: "AtomicGamer9523"@github.com
//LICENSE: "MIT"
//FORMAT: "DECLAREATIVE TS"
import * as blake2 from './blake2b';


class Block {
    previos_hash: string;
    data: string;
    constructor(data: string, chain: Chain){
        this.previos_hash = chain.hash;
        this.data = data;
    }
    static _initial(data: string){
        return new Block(data,new Chain())
    }
}

class Chain {
    hash: string;
    constructor(initial_block_data: string){
        this.hash = blake2.blake2b(Block._initial(initial_block_data));
    }
    push(block: Block){
        this.hash = blake2.blake2b(block)
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
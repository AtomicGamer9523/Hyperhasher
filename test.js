const {Block, Chain} = require('./src/lib')

let chain = new Chain();

let block = new Block("DATA_FOR_THE_BLOCK",chain);

chain.push(block);

console.log("Chain:", chain);
console.log("Block:", block);
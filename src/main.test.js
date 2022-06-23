const{Block,Chain}=require('./lib');
let chain=new Chain("INITIALBLOCK");
let block=new Block("First Block",chain);
chain.push(block);
if(chain.hash!="03ca603605beef76943b39bdfda55f4240551b3260889ab4b34a295b91d6e68bae4859f5aa04a9a4ae8e305f6750d9627fc6d8011d8446a38a91d32c9f88f342"){
    throw new Error("BLAKE2_ERROR")
}
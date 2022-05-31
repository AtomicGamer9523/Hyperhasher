use hyperhasher::{Systems, Block, Chain, config};

fn main() {
    let mut systems = Systems::new();
    
    let conf = config("manifest");
    let mut chain = Chain::new(conf);
    systems.add(Box::new(chain));

    let block = Block::new("lol".to_owned(), &chain);
    chain.push(block);
    systems.run();
}
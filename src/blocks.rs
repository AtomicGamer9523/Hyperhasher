//AUTHOR: "AtomicGamer9523"@github.com
//FORMAT: "RUST"
use std::hash::{Hash};
use std::fs;
use chrono;
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Block {
    pub previos_hash: u64,
    pub data: String
}
impl Block {
    #![allow(dead_code)]
    pub fn new(data: String, chain: &Chain) -> Self {
        Self {
            previos_hash: chain.hash,
            data: data
        }
    }
    pub fn initial(data: String) -> Self {
        Self {
            previos_hash: 0,
            data: data
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Copy)]
pub struct Chain {
    pub hash: u64,
    debug: bool
}
impl Chain {
    #![allow(dead_code)]
    pub fn new(config: crate::config::Config) -> Self {
        let debug: bool = config.debugmode;
        if debug {
            crate::info!(format!("\x1b[38;5;36mConfig: \x1b[0m{:?}\n",&config));
        }
        if config.autoloadsavedhash {
            if debug {
                crate::info!("\x1b[38;5;36m'autoloadsavedhash' is enabled, trying to laod hash file\x1b[0m\n");
            }
            match fs::read_to_string("hash") {
                Ok(data) => {
                    let hash = data.parse::<u64>().unwrap();
                    if debug {
                        crate::info!(format!("\x1b[38;5;36mhash file loaded, hash: \x1b[38;5;91m'{}'\x1b[0m\n", &hash));
                    }
                    Self {
                        hash: hash,
                        debug: debug
                    }
                },
                _ => {
                    let (hash, _) = crate::hash!(Block::initial(config.initialblockdata));
                    if debug {
                        crate::info!(format!("\x1b[38;5;36mno hash file found, creating a new \x1b[38;5;126mINITIAL_BLOCK\x1b[38;5;36m, hash: '\x1b[38;5;91m{}\x1b[38;5;36m'\x1b[0m\n", &hash));
                        crate::info!(format!("\x1b[38;5;126mINITIAL_BLOCK\x1b[38;5;167m -> \x1b[38;5;68m{}\x1b[0m\n", &hash));
                    }
                    Self {
                        hash: hash,
                        debug: debug
                    }
                }
            }
        } else {
            let (hash, _) = crate::hash!(Block::initial(config.initialblockdata));
            if debug {
                crate::info!(format!("\x1b[38;5;36m'autoloadsavedhash' is disabled, creating a new \x1b[38;5;126mINITIAL_BLOCK\x1b[38;5;36m, hash: \x1b[38;5;91m'{}'\x1b[0m\n", &hash));
                crate::info!(format!("\x1b[38;5;126mINITIAL_BLOCK\x1b[38;5;167m -> \x1b[38;5;68m{}\x1b[0m\n", &hash));
            }
            Self {
                hash: hash,
                debug: debug
            }
        }
    }
    pub fn push(&mut self, block: Block) -> u64 {
        let (blockhash, oldblock) = crate::hash!(block);
        if self.debug==true{
            crate::info!(format!("\x1b[38;5;68m{}\x1b[38;5;167m + \x1b[38;5;68m{:?}\x1b[0m \x1b[38;5;167m-> \x1b[38;5;68m{}\x1b[0m\n",&oldblock.previos_hash,&oldblock.data, &blockhash));
        };
        self.hash = blockhash;
        self.hash
    }
    pub fn clean(&mut self){
        match fs::write("hash",format!("{}",self.hash).as_bytes()) {
            Ok(_) => {},
            Err(e) => {
                crate::error!(format!("Chain Saving Error: '{}'", e));
            }
        }
    }
}
impl crate::systems::System for Chain {
    fn init(&mut self) { }
    fn tick(&mut self) { }
    fn box_clone(&self) -> Box<dyn crate::systems::System> {
        Box::new((*self).clone())
    }
}
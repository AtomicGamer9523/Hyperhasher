//AUTHOR: "AtomicGamer9523"@github.com
//FORMAT: "RUST"
use ron::de::from_reader;
use serde::Deserialize;
use std::fs::{File};
#[derive(Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd )]
pub struct Config {
    pub initialblockdata: String,
    pub autoloadsavedhash: bool,
    pub usedotlogfile: bool,
    pub debugmode: bool
}
impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
pub fn config(file: &'static str) -> Config {
    let src = match File::open(file) {
        Ok(data) => {data},
        Err(_)=>{
            crate::error!(format!("Error loading file: '{}'",  &file));
        }
    };
    match from_reader(src) {
        Ok(x) => x,
        Err(e) => {
            crate::error!(format!("Failed to load config: {}",&e));
        }
    }
}
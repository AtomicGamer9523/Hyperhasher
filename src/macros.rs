//AUTHOR: "AtomicGamer9523"@github.com
//FORMAT: "RUST"
#[macro_export]
macro_rules! hash {
    (
        $e: expr
    ) => {
        {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hasher;
            let t = $e;
            let mut hash = DefaultHasher::new();
            t.hash(&mut hash);
            (
                hash.finish(),
                t
            )
        }
    };
}
#[macro_export]
macro_rules! info {
    (
        $e:expr
    ) => {
        {
            let t = $e;
            print!("\x1b[38;5;236m{} \x1b[38;5;92m\x1b[1mINFO\x1b[0m \x1b[38;5;0m[\x1b[38;5;24m{}\x1b[38;5;0m]\x1b[0m {}",chrono::offset::Local::now().format("%Y/%m/%d %H:%M:%S,%6f"), env!("CARGO_PKG_NAME"), t);
        }
    };
}
#[macro_export]
macro_rules! debug {
    (
        $e: expr
    ) => {
        {
            let t = $e;
            print!("\x1b[38;5;236m{} \x1b[38;5;26m\x1b[1mDEBUG\x1b[0m \x1b[38;5;0m[\x1b[38;5;24m{}\x1b[38;5;0m]\x1b[0m {}",chrono::offset::Local::now().format("%Y/%m/%d %H:%M:%S,%6f"), env!("CARGO_PKG_NAME"), t);
        }
    };
}
#[macro_export]
macro_rules! warn {
    (
        $e: expr
    ) => {
        {
            let t = $e;
            print!("\x1b[38;5;236m{} \x1b[38;5;166m\x1b[1mWARN\x1b[0m \x1b[38;5;0m[\x1b[38;5;24m{}\x1b[38;5;0m]\x1b[0m {}",chrono::offset::Local::now().format("%Y/%m/%d %H:%M:%S,%6f"), env!("CARGO_PKG_NAME"), t);
        }
    };
}
#[macro_export]
macro_rules! error {
    (
        $e: expr
    ) => {
        {
            let t = $e;
            print!("\x1b[38;5;236m{} \x1b[38;5;1m\x1b[1mERROR\x1b[0m \x1b[38;5;0m[\x1b[38;5;24m{}\x1b[38;5;0m]\x1b[0m {}",chrono::offset::Local::now().format("%Y/%m/%d %H:%M:%S,%6f"), env!("CARGO_PKG_NAME"), t);
            std::process::exit(1);
        }
    };
}
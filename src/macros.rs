//AUTHOR: "AtomicGamer9523"@github.com
//LICENSE: "MIT"
//FORMAT: "RUST"
#[macro_export]
macro_rules! hash {
    ( $e: expr ) => {{
        use blake2::{Digest, Blake2b512};
        use std::convert::TryInto;
        let mut hasher = Blake2b512::new();
        hasher.update(
            format!(
                "{}",
                $e
            )
            .as_bytes()
        );
        let hashres: [u8;64] = hasher.finalize()
        .as_slice().try_into().expect("Wrong Length");
        crate::HashString::from_generic_array(hashres)
    }};
}
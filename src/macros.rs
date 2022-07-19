//AUTHOR: "AtomicGamer9523"@github.com
//LICENSE: "MIT"
//FORMAT: "RUST"
#[macro_export]
macro_rules! hash {
    ( $e: expr ) => {{
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(
            format!(
                "{}",
                $e
            )
            .as_bytes()
        );
        crate::HashString::from(hasher.finalize())
    }};
}
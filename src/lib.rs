// pub mod ctp {
//     include!(concat!(env!("OUT_DIR"), "/ctp_bindings.rs"));
// }

// Automatically include all modules from src directory
mod auto_modules {
    macro_rules! include_modules {
        () => {
            // Include files directly in src/
            include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/mod.rs"));
        };
    }
    include_modules!();
}
pub use auto_modules::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub mod ctp {
    include!(concat!(env!("OUT_DIR"), "/ctp_bindings.rs"));
}

pub mod atp {
    include!(concat!(env!("OUT_DIR"), "/atp_bindings.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
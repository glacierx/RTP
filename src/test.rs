#[cfg(test)]

// #[test]
// fn create_apt_instance() {
//     use crate::atp::trader::*;
//     let flow_path = ::std::ffi::CString::new("").unwrap();
//     let api = TraderApi::new(flow_path);
// }

#[test]
fn create_ctp_instance() {
    use super::trader::*;
    let flow_path = ::std::ffi::CString::new("").unwrap();
    let api = TraderApi::new(flow_path);
    drop(api);
}
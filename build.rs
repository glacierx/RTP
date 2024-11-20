use std::env;
use std::path::PathBuf;
use std::fs;

const CTP_SO_NAME: &str = "thosttraderapi_ctp.so";
const ATP_SO_NAME: &str = "thosttraderapi_atp.so";

fn generate_bindings(api_type: &str, header_file: &str, out_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bindings = bindgen::Builder::default()
        .header(header_file)
        .derive_debug(false)
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR")?).join(out_file);
    bindings.write_to_file(out_path)?;
    
    Ok(())
}

fn setup_api(api_type: &str, so_name: &str, orig_name: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let current_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    
    // Copy and rename library
    let source = format!("{}/libs/{}/lib/{}", current_dir, api_type, orig_name);
    let target = format!("{}/{}", out_dir, so_name);
    fs::copy(&source, &target).unwrap_or_else(|_| panic!("Failed to copy {} library", api_type));
    
    // Generate bindings
    let header = format!("{}/libs/{}/include/ThostFtdcUserApiStruct.h", 
        current_dir, api_type);
    
    generate_bindings(
        api_type,
        &header,
        &format!("{}_bindings.rs", api_type)
    ).unwrap_or_else(|e| panic!("Failed to generate {} bindings: {}", api_type, e));
    
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rerun-if-changed={}", source);
}

fn main() {
    // setup_api("ctp", CTP_SO_NAME, "libthosttraderapi.so");
    setup_api("atp", ATP_SO_NAME, "libthosttraderapi.so");
}
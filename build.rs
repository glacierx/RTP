use encoding::all::GB18030;
use encoding::{decode, DecoderTrap};
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::path::PathBuf;
use xmltree::{Element, XMLNode};

#[cfg(feature="ctp")]
const SO_NAME: &str = "libthosttraderapi_ctp.so";

#[cfg(feature="atp")]
const SO_NAME: &str = "libthosttraderapi_atp.so";

pub fn gb18030_bytes_to_string(bytes: &[u8]) -> String {
    decode(bytes, DecoderTrap::Replace, GB18030)
        .0
        .unwrap_or_else(|e| e.into_owned())
}

#[derive(Debug)]
struct ErrorEntry {
    id: String,
    value: i64,
    prompt: String,
}

#[derive(Debug)]
struct Errors {
    errors: Vec<ErrorEntry>,
}

impl Errors {
    pub fn from_xml_element(element: Element) -> Result<Self, String> {
        let mut errors = Vec::new();
        for child in element.children {
            // Convert XMLNode to Element if possible
            if let XMLNode::Element(child_elem) = child {
                let id = child_elem
                    .attributes
                    .get("id")
                    .ok_or(String::from("no id attribute in one of the child"))?
                    .to_owned();

                let value_string = child_elem
                    .attributes
                    .get("value")
                    .ok_or(String::from("no value attribute in one of the child"))?;

                let value = value_string
                    .parse()
                    .map_err(|e| format!("cannot parse value to integer, {}", e))?;

                let prompt = child_elem
                    .attributes
                    .get("prompt")
                    .ok_or(String::from("no prompt attribute in one of the child"))?
                    .to_owned();

                errors.push(ErrorEntry { id, value, prompt });
            }
        }
        Ok(Errors { errors })
    }
}

fn generate_error(input_xml: &Path, output_rs: &Path) -> Result<(), String> {
    let mut file_bytes = vec![];
    let mut input_file = std::fs::File::open(input_xml)
        .map_err(|e| format!("failed to open data_type header, {}", e))?;
    input_file
        .read_to_end(&mut file_bytes)
        .map_err(|e| format!("filed to read data_type header, {}", e))?;

    let file_string = gb18030_bytes_to_string(&file_bytes);
    let element = Element::parse(file_string.as_bytes())
        .map_err(|e| format!("failed to parse input file as xml, {}", e))?;

    let errors = Errors::from_xml_element(element)
        .map_err(|e| format!("cannot generate errors from parsed xml element, {}", e))?;

    let mut error_output = std::io::BufWriter::new(
        std::fs::File::create(output_rs).map_err(|e| format!("cannot create error file, {}", e))?,
    );

    for error in errors.errors.iter() {
        error_output
            .write(
                format!(
                    "pub const ERROR_{}: TThostFtdcErrorIDType = {};\n",
                    error.id, error.value
                )
                .as_bytes(),
            )
            .map_err(|e| format!("cannot write error file, {}", e))?;
    }

    error_output
        .write(b"pub fn error_id_to_chinese_description(error_id: TThostFtdcErrorIDType) -> &'static str {\n")
        .map_err(|e| format!("cannot write error file, {}", e))?;

    error_output
        .write(b"    match error_id {\n")
        .map_err(|e| format!("cannot write error file, {}", e))?;

    for error in errors.errors.iter() {
        error_output
            .write(format!("        ERROR_{} => \"{}\",\n", error.id, error.prompt).as_bytes())
            .map_err(|e| format!("cannot write error file, {}", e))?;
    }

    error_output
        .write(b"        _ => \"unknown error\",\n")
        .map_err(|e| format!("cannot write error file, {}", e))?;
    error_output
        .write(b"    }\n}\n")
        .map_err(|e| format!("cannot write error file, {}", e))?;

    Ok(())
}

fn generate_bindings(
    header_file: &str,
    out_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let bindings = bindgen::Builder::default()
        .header(header_file)
        // .use_core()
        .derive_default(true)
        .derive_debug(false)
        .layout_tests(false)
        // .formatter(bindgen::Formatter::Prettyplease)
        // .layout_tests(false)
        // .allowlist_recursively(true)
        // .allowlist_type("CThostFtdc.*")
        // .allowlist_function(".*CreateFtdc.*")
        // .allowlist_var("THOST_.*")
        .rustified_enum(".*") // Convert all enums into idiomatic Rust enums
        .allowlist_type(".*") // Include all types
        .allowlist_function(".*") // Include all functions
        .allowlist_var(".*") // Include all variables (e.g., constants)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join(out_file);
    let binding_output = bindings.to_string().replace("c_char", "c_uchar");
    let mut output_file =
        std::fs::File::create(out_path).map_err(|e| format!("cannot create struct file, {}", e))?;
    output_file
        .write_all(binding_output.as_bytes())
        .map_err(|e| format!("cannot write struct file, {}", e))
        .unwrap();
    Ok(())
}

fn setup_api(api_type: &str, so_name: &str, orig_name: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let current_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let source = format!("{}/libs/{}/linux64/{}", current_dir, api_type, orig_name);
    let target = format!("{}/{}", out_dir, so_name);
    fs::copy(&source, &target).unwrap_or_else(|_| panic!("Failed to copy {} library", api_type));

    let headers = vec!["ThostFtdcUserApiStruct.h"];
    let manifest_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    for filename in headers {
        let header = format!("{}/libs/{}/include/{}", current_dir, api_type, filename);
        // let prefix_name = filename.split(".").next().unwrap();
        generate_bindings(
            &header,
            &format!(
                "{}/src/{}/generated/{}_structs.rs",
                manifest_path, api_type, api_type
            ),
        )
        .unwrap_or_else(|e| panic!("Failed to generate {} bindings: {}", api_type, e));
        let input_xml = PathBuf::from(format!("{}/libs/{}/misc/error.xml", current_dir, api_type));
        let output_error_rs = PathBuf::from(format!(
            "{}/src/{}/generated/{}_errors.rs",
            manifest_path, api_type, api_type
        ));
        generate_error(input_xml.as_path(), output_error_rs.as_path())
            .unwrap_or_else(|e| panic!("Failed to generate {} bindings: {}", api_type, e));
    }

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rerun-if-changed={}", source);
}

fn generate_trader_api(api_type: &str, so_name: &str, orig_name: &str) {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let current_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    let target_so = format!("{}/{}", out_dir, so_name);
    std::fs::copy(
        &format!("{}/libs/{}/linux64/{}", current_dir, api_type, orig_name),
        &target_so,
    )
    .expect("failed to copy so to outdir");
    println!("cargo:resource={}", target_so);
}

fn main() {
    let features = vec![
        cfg!(feature = "ctp"),
        cfg!(feature = "atp"),
        cfg!(feature = "xtp"),
    ];
    
    let enabled_count = features.iter().filter(|&&x| x).count();
    if enabled_count > 1 {
        panic!("Only one SDK feature can be enabled at a time");
    }
    if enabled_count == 0 {
        panic!("At least one SDK feature must be enabled");
    }    
    // setup_api("ctp", CTP_SO_NAME, "libthosttraderapi.so");
    #[cfg(feature="atp")]
    setup_api("atp", SO_NAME, "libthosttraderapi.so");
    #[cfg(feature="atp")]
    generate_trader_api("atp", SO_NAME, "libthosttraderapi.so");
    #[cfg(feature="ctp")]
    setup_api("ctp", SO_NAME, "thosttraderapi_se.so");
    #[cfg(feature="ctp")]
    generate_trader_api("ctp", SO_NAME, "thosttraderapi_se.so");
    // // Generate mod.rs
    // if let Err(e) = generate_recursive_mods() {
    //     eprintln!("Warning: Failed to generate mod.rs: {}", e);
    // }

    // Add this to make cargo rebuild when source files change
    println!("cargo:rerun-if-changed=src");
}

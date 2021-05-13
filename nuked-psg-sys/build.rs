use std::{env, path::PathBuf};

fn main() {
    let vendor_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("vendor");
    let nuked_psg_dir = vendor_dir.join("Nuked-PSG");

    cargo_config();
    bindings(&nuked_psg_dir);
    build(&nuked_psg_dir);
}

fn cargo_config() {
    println!("cargo:rustc-link-lib=static=nuked-psg");
}

fn build(source_dir: &PathBuf) {
    cc::Build::new()
        .file(&source_dir.join("ympsg.c"))
        .compile("nuked-psg");
}

fn bindings(source_dir: &PathBuf) {
    let header_file = source_dir.join("ympsg.h");
    let bindings_file = PathBuf::from(env::var("OUT_DIR").unwrap()).join("nuked-psg-bindings.rs");

    bindgen::Builder::default()
        .header(header_file.to_str().unwrap())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(bindings_file)
        .expect("Couldn't write bindings!");
}

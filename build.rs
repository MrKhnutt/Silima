#![allow(non_snake_case)]

// use std::{fs, os::unix::ffi::OsStrExt};
use blake3;
use walkdir::WalkDir;
use sha2::{Digest, Sha256};
use std::{
    ffi::{OsString}, 
    fs, 
    os::unix::ffi::OsStrExt,
};



fn createDirFile(dir: &str, fileNames: &Vec<OsString>) {
    
    let testString: String = 
        fileNames.iter()
            .map(|name| format!("/{}", name.to_string_lossy()))
            .collect::<Vec<_>>()
            .join("\n");

    fs::write(format!("{dir}/directory.dat"), testString)
        // .unwrap()
        .expect("failed to write to file");

    // };
}

fn main() {
    // println!("cargo::rerun-if-changed=src");
    // println!("cargo::rerun-if-changed=static");
    // println!("cargo::rerun-if-changed=admin");

    const CLIENT_DIR: &str  = "static";
    const ADMIN_DIR : &str  = "admin";
    const SRC_DIR   : &str  = "src";


    let mut rsHash = blake3::Hasher::new();
    let mut jsHash = Sha256::new();
    let mut adHash = Sha256::new();
        
    let mut fileNames: Vec<OsString> = Vec::new();
    for entry in WalkDir::new(SRC_DIR).sort_by_file_name() {
        match entry {
            Ok(filePath) => {
                if filePath.file_type().is_file() {
                    fileNames.push(filePath.file_name().to_os_string());
                    rsHash.update(filePath.file_name().as_bytes());
                    rsHash.update(&fs::read(filePath.path()).unwrap());
            }}
            Err(_) => {}
    }};
    createDirFile(SRC_DIR, &fileNames);
    fileNames.clear();

    // createDirFile("static");
    for entry in WalkDir::new(CLIENT_DIR).sort_by_file_name() {
        match entry {
            Ok(filePath) => {
                if filePath.file_type().is_file() {
                    fileNames.push(filePath.file_name().to_os_string());
                    jsHash.update(filePath.file_name().as_bytes());
                    jsHash.update(&fs::read(filePath.path()).unwrap());
            }}
            Err(_) => {}
    }};
    // createDirFile(CLIENT_DIR, &fileNames);
    fileNames.clear();

    for entry in WalkDir::new(ADMIN_DIR).sort_by_file_name() {
        match entry {
            Ok(filePath) => {
                if filePath.file_type().is_file() {
                    fileNames.push(filePath.file_name().to_os_string());
                    adHash.update(filePath.file_name().as_bytes());
                    adHash.update(&fs::read(filePath.path()).unwrap());
            }}
            Err(_) => {}
    }};
    createDirFile(ADMIN_DIR, &fileNames);
    fileNames.clear();

    println!(
        "cargo::rustc-env=BUILD_RS_HASH={}", 
        &rsHash.finalize().to_hex().to_string()[..4]
    );
    println!(
        "cargo::rustc-env=BUILD_JS_HASH={}", 
        &hex::encode(jsHash.finalize())[..4]
    );
    println!(
        "cargo::rustc-env=BUILD_AD_HASH={}", 
        &hex::encode(adHash.finalize())[..4]
    );
}
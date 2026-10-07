#![allow(non_snake_case)]

use std::{fs, os::unix::ffi::OsStrExt};
use blake3;
use walkdir::WalkDir;
use sha2::{Digest, Sha256};

fn main() {
    println!("cargo::rerun-if-changed=src");
    println!("cargo::rerun-if-changed=static");

    let mut rsHash = blake3::Hasher::new();
    let mut jsHash = Sha256::new();
    let mut adHash = Sha256::new();

    for entry in WalkDir::new("src").sort_by_file_name() {
        match entry {
            Ok(filePath) => {
                if filePath.file_type().is_file() {
                    rsHash.update(filePath.file_name().as_bytes());
                    rsHash.update(&fs::read(filePath.path()).unwrap());
            }}
            Err(_) => {}
    }};
    for entry in WalkDir::new("static").sort_by_file_name() {
        match entry {
            Ok(filePath) => {
                if filePath.file_type().is_file() {
                    jsHash.update(filePath.file_name().as_bytes());
                    jsHash.update(&fs::read(filePath.path()).unwrap());
            }}
            Err(_) => {}
    }};
    for entry in WalkDir::new("admin").sort_by_file_name() {
        match entry {
            Ok(filePath) => {
                if filePath.file_type().is_file() {
                    adHash.update(filePath.file_name().as_bytes());
                    adHash.update(&fs::read(filePath.path()).unwrap());
            }}
            Err(_) => {}
    }};

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
#![allow(non_snake_case)]
#![allow(dead_code)]

use sha2::{Digest, Sha256};
use std::{
    fs, 
    io::Read,
};

enum Extensions {
    One(String),
    Many(Vec<String>),
}
struct ManifestOptions {
    save    : bool,
    rerun   : bool,
    hash    : bool,
    write   : bool,
    
    filter  : Option<Extensions>,
} impl Default for ManifestOptions {
    fn default() -> Self {
        Self {
            save    : true,
            rerun   : true,
            hash    : true,
            write   : true,

            filter : None,
        }
    }
}

fn createManifest(dir: &str, config: ManifestOptions) -> Result<String, std::io::Error>{
    const KB        : usize    = 8192;
    let hashHex     : String;

    // collect all files
    let mut fileNames = std::fs::read_dir(dir)
        .expect("failed to find directory")
        .filter_map(Result::ok)
        .filter_map(|entry| 
                entry.file_name().into_string().ok())
        .collect::<Vec<String>>();

    // filter out unwanted files
    match config.filter {
        // keep only the files with extension X
        Some(Extensions::One(value)) => {
            fileNames = fileNames
                .into_iter()
                .filter(|entry| entry.contains(&value))
                .collect();
        }
        // remove the files that have extensions X, Y and Z
        Some(Extensions::Many(values)) => {
            for ext in &values {
                fileNames = fileNames
                    .into_iter()
                    .filter(|entry| !entry.contains(ext))
                    .collect();
            };
        }
        // do nothing
        None => {}
    };
    fileNames.sort();
    
    // hash all files
    if config.hash {
        let mut hash = Sha256::new();

        for file in &fileNames {
            let mut byteBuffer = [0u8; KB];
                let mut bytes = 
                        fs::File::open(
                            std::path::Path::new(dir).join(file)
                        )?;
            loop {
                let bytesRead = bytes.read(&mut byteBuffer)?;
                // file name THEN contents
                hash.update(file.as_bytes());
                hash.update(byteBuffer);
                if bytesRead == 0 
                    // EOF
                    { break; }
            };
        };
        // keep 4 characters
        hashHex = hex::encode(hash.finalize())[..4].to_string();
    } else 
    // No hash generated
    { hashHex = "".to_string(); }

    // setup system variables
    if config.rerun
        { println!("cargo::rerun-if-changed={dir}"); }
    if config.save && config.hash {
        println!(
            "cargo::rustc-env=BUILD_{}_HASH={}",
            dir.to_uppercase(),
            hashHex,
        );
    }

    // write manifest
    if config.write {
        let manifest = serde_json::json!({
            "hash"  :   hashHex,
            "files" :   fileNames,
        });
        fs::write(
            format!("{dir}/manifest.json"),
            manifest.to_string()
        )?;
    };
    // return that it worked
    Ok(hashHex.to_string())
}

fn main() {
    const SRC_DIR   : &str  = "src";
    const CLIENT_DIR: &str  = "client-controller";
    const ADMIN_DIR : &str  = "admin";
    const BG_DIR    : &str  = "media/background";

    match createManifest(SRC_DIR, ManifestOptions { 
        write: false,
        ..Default::default()
    }){
        Ok(_) => {}
        Err(e) => {
            println!("cargo::error=Manifest failed on {SRC_DIR}: {e}");
        }
    };
    match createManifest(CLIENT_DIR, ManifestOptions { 
        ..Default::default()
    }){
        Ok(_) => {}
        Err(e) => {
            println!("cargo::error=Manifest failed on {CLIENT_DIR}: {e}");
        }
    };
    match createManifest(ADMIN_DIR, ManifestOptions { 
        ..Default::default()
    }){
        Ok(_) => {}
        Err(e) => {
            println!("cargo::error=Manifest failed on {ADMIN_DIR}: {e}");
        }
    };
    match createManifest(BG_DIR, ManifestOptions {
        save    : false,
        hash    : false,
        filter  : Some(Extensions::One("svg".to_string())),
        ..Default::default()
    }){
        Ok(_) => {}
        Err(e) => {
            println!("cargo::error=Manifest failed on {BG_DIR}: {e}");
        }
    };
}
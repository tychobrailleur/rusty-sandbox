use sha2::{Sha256, Digest};
use num::bigint::{BigInt, Sign};
use std::io;
use std::fs::File;
use std::path::Path;


fn display_hash(s:String) {
     let mut hasher = Sha256::new();
    hasher.input(s.as_bytes());

    let result = hasher.result();

    println!();
    println!("ID = {}", BigInt::from_bytes_be(Sign::Plus, &result));
    // Formatting traits: https://doc.rust-lang.org/std/fmt/#formatting-traits
    println!("Hash = {:x}", result);
}

fn main() {

    let strs = [
        "abc",
        "abc "
    ];

    for s in strs.iter() {
        display_hash(s.to_string());
    }

    let mut hasher = Sha256::new();
    let path = Path::new("./etc/example.pdf");
    let mut file = match File::open(&path) {
        Err(e) => panic!("couldn't open file: {}", e),
        Ok(file) => file
    };
    io::copy(&mut file, &mut hasher).unwrap();
    let result = hasher.result();

    println!("ID = {}", BigInt::from_bytes_be(Sign::Plus, &result));
    println!("Hash = {:x}", result);
}

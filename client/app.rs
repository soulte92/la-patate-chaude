mod hash_collection;


use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let message = b"000000000000034Chello";

    let digest = md5::compute(message);

    println!("My HashCode Generated by MD5 is: {:x}", digest) ;
}

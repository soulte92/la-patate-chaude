extern crate data_encoding;
extern crate hex;
extern crate md5;
extern crate rand;
extern crate rayon;

use std::convert::{TryFrom, TryInto};

use data_encoding::BASE64;
use md5::Digest;
use rand::{RngCore, thread_rng};
use crate::data_structures::MD5HashCashInput;
use crate::hashcash::md5_hascash_challenge::{Challenge, Md5Challenge};


fn main() {
    let md5_hashcash_input = MD5HashCashInput {
        complexity: 9,
        message: "hello".to_string()
    };

    let md5_hashcash_challenge = Md5Challenge::new(md5_hashcash_input);
    let result = md5_hashcash_challenge.solve();

    let status = md5_hashcash_challenge.verify(&result);

    println!("Validation status: {status}")
}
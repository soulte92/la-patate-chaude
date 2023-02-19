extern crate random_string;

use std::borrow::Borrow;
use std::io::prelude::*;
use std::net::TcpStream;

mod recover_secret;

use recover_secret::data_structures::{RecoverSecretInput, RecoverSecretOutput};
use recover_secret::recover_secret_challenge::{Challenge, RecoverSecretChallenge};

fn main() {
    let first_test_word = "C'est chou";
    let recover_secret_output = RecoverSecretOutput{
        secret_sentence: first_test_word.to_string()
    };

    let recover_secret_input = RecoverSecretInput {
        word_count: 2,
        letters: "t cCehuCethoCeschouC'schout h".to_string(),
        tuple_sizes: Vec::from([3, 4, 5, 7, 7, 3])
    };

    let challenge_recover_secret = RecoverSecretChallenge::new(recover_secret_input);

    let status = challenge_recover_secret.verify(&recover_secret_output);
    let out = challenge_recover_secret.solve();
    // println!("{}", out.secret_sentence);
    println!("{}", status);



}
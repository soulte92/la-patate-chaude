use std::borrow::Borrow;

use data_encoding::BASE64;
use rand::{RngCore, thread_rng};

pub fn count_zero_bits(hash: &str, complexity: u32) -> u32 {
    let bytes = hex::decode(hash).unwrap();
    let mut num_leading_zero_bits = 0;
    for byte in &bytes {
        if byte.leading_zeros() >= complexity {
            num_leading_zero_bits += complexity;
        } else {
            num_leading_zero_bits += byte.leading_zeros();
            break;
        }
    }
    num_leading_zero_bits + 1
}

pub fn generated_md5_from_string(message: String) -> String {
    let digest = md5::compute(message);
    let hash_to_test = format!("{:x}", digest);
    hash_to_test.to_uppercase()
}

// Get the index of letter parameter in word parameter as parameters
pub fn get_value_index(word: String, letter: String) -> i32 {
    for j in 0..word.len(){
        if word.chars().nth(j).unwrap() == letter.chars().nth(0).unwrap(){
            return j as i32;
        }
    }
    return -1;
}

// Verify if a Vector of i32 is sorted
pub fn is_sorted<T>(data: Vec<i32>) -> bool
    where
        T: Ord,
{
    data.windows(2).all(|w| w[0] <= w[1])
}

// Verify if word parameter repects pattern variable
pub fn pattern_word(pattern:String, word: String) -> bool {
    // Check contains all need characters
    let mut result: Vec<i32> = Vec::new();
    for i in 0..pattern.len(){
        // Add indexes of found letters in result variable
        for j in 0..word.len(){
            if word.chars().nth(j).unwrap() == pattern.chars().nth(i).unwrap(){
                result.push(j as i32);
            }
        }
    }
    // Check we have the letters in the correct order
    is_sorted::<u8>(result)
}
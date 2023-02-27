extern crate random_string;

use std::borrow::Borrow;
use std::io::prelude::*;
use std::net::TcpStream;

pub mod recover_secret;
pub mod hashcash;
mod recover_secret_test;
mod md5_hashcash_test;


use shared::data_structures::{
    ChallengeAnswer, ChallengeResult, ChallengeTrait, Message, PublicLeaderBoard,
    Subscribe,
};
use hashcash::md5_hascash_challenge::Md5Challenge;
use std::env;
use recover_secret::recover_secret_challenge::{RecoverSecretChallenge};
use shared::broadcasters;
use shared::data_structures::{Challenge, RecoverSecretInput, RecoverSecretOutput};

fn main() {
    // recover_secret_main();
    run()
}

fn run() {
    let args: Vec<String> = env::args().collect();
    let address;

    if args.len() != 2 {
        address = "127.0.0.1:7878";
    } else {
        address = &args[1];
    }

    let mut stream = TcpStream::connect(address).unwrap();

    let player_name: String = "Sacha".to_string();
    let mut leaderboard: PublicLeaderBoard = PublicLeaderBoard(Vec::new());

    broadcasters::send_message(&mut stream, Message::Hello);

    loop {
        let response = broadcasters::receive_message(&mut stream);

        match response {
            Message::Welcome(..) => {
                let subscribe = Subscribe {
                    name: player_name.clone(),
                };
                broadcasters::send_message(&mut stream, Message::Subscribe(subscribe));
            }
            Message::SubscribeResult(..) => {
                println!(" \n\x1b[32mSucessfully subscribed\x1b[0m");
                continue;
            }
            Message::PublicLeaderBoard(input) => {
                leaderboard = input;
            }
            Message::Challenge(challenge) => match challenge {
                Challenge::MD5HashCash(input) => {
                    let challenge = Md5Challenge::new(input);

                    let output = challenge.solve();

                    let result = Message::ChallengeResult(ChallengeResult {
                        answer: ChallengeAnswer::MD5HashCash(output),
                        next_target: get_next_target(&leaderboard, &player_name),
                    });

                    broadcasters::send_message(&mut stream, result);
                }
                Challenge::RecoverSecret(input) => {
                    let challenge = RecoverSecretChallenge::new(input);

                    let output = challenge.solve();

                    let result = Message::ChallengeResult(ChallengeResult {
                        answer: ChallengeAnswer::RecoverSecret(output),
                        next_target: get_next_target(&leaderboard, &player_name),
                    });

                    broadcasters::send_message(&mut stream, result);
                }
            },
            Message::ChallengeTimeout(..) => {
                println!(" \n\x1b[32mTimed out\x1b[0m");
                break;
            }
            Message::RoundSummary(..) => {
                println!(" \n\x1b[32mEnd of round\x1b[0m");
                continue;
            }
            Message::EndOfGame(..) => {
                println!(" \n\x1b[32mEnd of game\x1b[0m");
                break;
            }
            _ => {
                panic!("Something went wrong !");
            }
        }
    }
}

fn get_next_target(leaderboard: &PublicLeaderBoard, player_name: &String) -> String {
    let random_player = leaderboard
        .0
        .iter()
        .find(|player| player.name != *player_name);

    return match random_player {
        Some(player) => player.name.clone(),
        None => return String::new(),
    };
}
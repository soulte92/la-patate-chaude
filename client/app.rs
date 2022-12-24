use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    loop {

        let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
        let message = "try to speak with";

        println!("{}", message.len());
        let len = message.len() as u32;

        stream.write(&len.to_be_bytes()).unwrap(); // on écrit le préfixe (taille du prochain message)
        stream.write(message.as_bytes()).unwrap(); // puis le message en tant que tel

    }
}

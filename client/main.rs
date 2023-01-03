use std::io::prelude::*;
use std::net::{TcpStream};

const ADDRESS_SERVER: &str = "127.0.0.1:7878";
const HELLO_MESSAGE: &str = r#""Hello""#;
const SUBSCRIBE_MESSAGE: &str = r#"{"Subscribe":{"name":"Sarah"}}"#;

fn main() {
    let stream = TcpStream::connect(ADDRESS_SERVER).unwrap();
        send_hello_to_server(&stream, HELLO_MESSAGE);
        send_subscribe_to_server(&stream, SUBSCRIBE_MESSAGE);
    /* TODO ⬇️
        while the the response of the server of the server
        isn't "EndOfGame", i listen the server response.
     */
    /*
        use std::io::Read;
        use std::net::{TcpListener, TcpStream};

        fn handle_client(mut stream: TcpStream) {
            let mut buf_len = [0u8; 4]; // pour lire les 4 octets du u32
            stream.read_exact(buf_len.as_mut()).unwrap(); // lit exactement la taille du buffer

            let len = u32::from_le_bytes(buf_len); // convertit les 4 octets en un entier u32

            let mut buf = vec![0u8; len as usize]; // on prépare un buffer pour ce qui va arriver
            stream.read_exact(buf.as_mut()).unwrap(); // on remplit le buffer
            // c'est arrivé
            println!("{buf:?}"); // en octets
            let s = String::from_utf8_lossy(&buf); // la version loosy n'échoue jamais et nettoie les caractères UTF-8 invalides
            println!("{s}"); // en String
        }

        fn main() {
            let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

            for stream in listener.incoming() {
                handle_client(stream.unwrap());
            }
        }
    */
}

fn send_hello_to_server(mut stream: &TcpStream, message: &str) {
    // Print it for us.
    println!("{}", message.len());
    // Store the length of our message.
    let length = message.len() as u32;
    // We write the prefix which is the size of the next message.
    stream.write(&length.to_be_bytes()).unwrap();
    // We write the message.
    stream.write(message.as_bytes()).unwrap();
}

fn send_subscribe_to_server(mut stream: &TcpStream, message: &str) {
    println!("{}", message.len());
    let length= message.len() as u32;
    stream.write(&length.to_be_bytes()).unwrap();
    stream.write(message.as_bytes()).unwrap();
}
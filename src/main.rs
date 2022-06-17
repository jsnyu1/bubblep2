//Create an executable that accepts two arguments either. The first argument

//is the type.




use std::env;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::io;
use std::net::UdpSocket;


fn handle_connection(mut stream: TcpStream) {
    let mut buffer:[u8; 5] = [0 ; 5];

    let mut ping:[u8;5] = ['p' as u8,'i' as u8,'n' as u8,'g' as u8, '!' as u8];

    let mut pong:[u8;5] = ['p' as u8,'o' as u8,'n' as u8,'g' as u8, '!' as u8];

    let mut error:[u8;5] = ['e' as u8, 'r' as u8 , 'r' as u8 ,'o' as u8 ,'r' as u8];

    stream.read(&mut buffer).unwrap();

    if buffer == ping {
        stream.write(&mut pong).unwrap();
    } else if buffer == pong {
        stream.write(&mut ping).unwrap();
    } else {
        stream.write(&mut error).unwrap();
    }

    stream.flush().unwrap();

    // let get = b"GET / HTTP/1.1\r\n";
    //
    // if buffer.starts_with(get) {
    //     // let contentsping = fs::read_to_string("ping.html").unwrap();
    //     // let contentspong = fs::read_to_string("pong.html").unwrap();
    //     // let contentserror = fs::read_to_string("error.html").unwrap();
    //     let response =
    //
    //         format!(
    //         // "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    //         contentspong.len(),
    //         contentspong
    //     );
    //     stream.write(response.as_bytes()).unwrap();
    //
    // } else {
    //
    //     // some other request
    // }
}


// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];

//     stream.read(&mut buffer).unwrap();

//     println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let protocol = &args[1];
    let address = &args[2];

    // println!("Ping? or Pong?");
    // let input = String::new();
    // let message = io::stdin().read_line(&mut input).expect("failed to readline");


    println!("Protocol: {}", protocol);
    println!("Address: {}", address);
    // println!("Message: {}", message);

    if protocol == "tcp" {
        let listener = TcpListener::bind(address).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established!");
            handle_connection(stream);

            //implemnt parallel connection
            // thread::spawn(|| {

            //     handle_connection(stream, message);

            // });
        }
    } else if protocol == "udp" {


        let socket = UdpSocket::bind(protocol).expect("couldn't find to address.");

        let mut buffer:[u8; 5] = [0 ; 5];
        let
        
    }
}

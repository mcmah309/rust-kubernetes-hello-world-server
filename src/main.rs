use std::io::Write;
use std::net::TcpListener;

fn main() {
    println!("Starting!");
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();

    // The listener's accept method waits or 'blocks' until
    // we have a connection and then returns a new TcpStream
    // that we can read and write data to.
    let message = "Hello, World!";
    let response = format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Type: text/html; charset=utf-8\r\n\
        Content-Length: {}\r\n\
        \r\n\
        {}",
        message.len(),
        message
    );
    for stream in listener.incoming() {
        println!("Stream recieved!");
        let mut stream = stream.unwrap();

        stream.write_all(response.as_bytes()).unwrap();
        println!("Response sent!");
    }
}

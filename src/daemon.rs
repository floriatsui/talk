use anyhow::Result;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

// TcpStream is an open socket that you can send data on
fn handle_client(mut stream: TcpStream) -> Result<()> {
    // TODO: initialize a thread
    println!("New connection! {:?}", stream);
    let mut buf = [0; 6];
    stream.read(&mut buf)?;

    println!("What we've read: {}", std::str::from_utf8(&buf)?);

    Ok(())
}

fn main() {
    if let Err(e) = run_daemon() {
        println!("Error: {}", e);
    }
}

fn run_daemon() -> Result<()> {
    // which computer to try and talk to a program on
    // one host might have multiple ips
    let ip = "0.0.0.0";
    // process
    let port = 8080;

    // Main thread that is always running and listening for connections
    let listener = TcpListener::bind((ip, port))?;

    // Initiate a thread per request
    for stream in listener.incoming() {
        let stream = stream?;

        // Thread that is going to run handle_client
        // Thread's function can't have arguments
        std::thread::spawn(|| {
            if let Err(e) = handle_client(stream) {
                println!("Error: {}", e);
            }
        });
    }
    Ok(())
}

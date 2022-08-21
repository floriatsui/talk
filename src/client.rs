use anyhow::Result;
use std::io::Write;
use std::net::TcpStream;

fn main() {
    println!("Hello from client");
    if let Err(e) = run_client() {
        println!("Error: {}", e);
    }
}

fn run_client() -> Result<()> {
    let daemon_ip = "127.0.0.1";
    let daemon_port = 8080;

    // Connect to the daemon
    // associated with TcpStream type
    let mut stream = TcpStream::connect((daemon_ip, daemon_port))?;
    stream.write("hello!".as_bytes())?;
    Ok(())
}

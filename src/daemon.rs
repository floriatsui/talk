use anyhow::Result;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "daemon", about = "The talk daemon")]
struct Opt {
    /// The host to bind
    #[structopt(short, long)]
    host: String,
    /// The port to run on
    #[structopt(short, long)]
    port: u16,
}

// TcpStream is an open socket that you can send data on
fn handle_client(mut stream: TcpStream) -> Result<()> {
    println!("New connection! {:?}", stream);
    let mut buf = [0; 6];
    stream.read(&mut buf)?;

    println!("What we've read: {}", std::str::from_utf8(&buf)?);

    // Loop:
    //   Read something from the socket
    //   find the appropriate connection from the group
    //   write using that connection yay
    Ok(())
}

fn main() {
    let args = Opt::from_args();
    if let Err(e) = run_daemon(args.host, args.port) {
        println!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_daemon(host: String, port: u16) -> Result<()> {
    // Main thread that is always running and listening for connections
    let listener = TcpListener::bind((host, port))?;

    // Initiate a thread per request
    for stream in listener.incoming() {
        let stream = stream?;

        // Thread that is going to run handle_client
        // Thread's function can't have arguments

        // do some bookkeeping
        std::thread::spawn(|| {
            if let Err(e) = handle_client(stream) {
                println!("Error: {}", e);
            }
        });
    }
    Ok(())
}

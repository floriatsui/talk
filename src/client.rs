use anyhow::Result;
use std::io::Write;
use std::net::TcpStream;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "client", about = "The talk client")]
struct Opt {
    /// The host where the daemon is running
    #[structopt(short, long)]
    host: String,
    /// The port where the daemon is listening
    #[structopt(short, long)]
    port: u16,
    /// Your username
    #[structopt(short, long)]
    username: String,
    /// The username of the other person to talk with
    #[structopt(short, long)]
    other: String,
}

fn main() {
    let args = Opt::from_args();

    if let Err(e) = run_client(args.host, args.port, args.username, args.other) {
        println!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_client(
    daemon_host: String,
    daemon_port: u16,
    username: String,
    other: String,
) -> Result<()> {
    // Connect to the daemon
    // associated with TcpStream type
    let mut stream = TcpStream::connect((daemon_host, daemon_port))?;

    stream.write("hello!".as_bytes())?;
    Ok(())

    // send an initial message: my username and then username of the opposite party

    // Spawn a thread that constantly reads from the socket and writes to terminal
    // What we get from the socket is from the opposite party

    // Loop:
    //   Read terminal input
    //   Write to socket
}

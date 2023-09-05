use std::net::Ipv4Addr;

use protocol::commands;
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub mod protocol;

// This is hard-coded for now
// TODO make this configurable to account for DHCP
const IP_ADDRESS: Ipv4Addr = Ipv4Addr::new(192, 168, 127, 26);

// TODO make this configurable
const PORT: u16 = 41794;

#[tokio::main]
async fn main() {
    // Connect to the projector
    let mut stream = TcpStream::connect((IP_ADDRESS, PORT)).await.unwrap();

    // Turn the projector on
    stream.write_all(&commands::power::ON).await.unwrap();
}

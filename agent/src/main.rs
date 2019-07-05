use std::env;

use std::io::Read;
use std::net::TcpStream;

use backupd::{Handshake, Ack};
use backupd::error as e;
use backupd::error::ResultExt;

fn client_start() -> e::Result<()> {
    // Load config
    let agent_name = env::var("AGENT_NAME")
        .chain_err(|| "Missing environment variable: AGENT_NAME")?;
    assert_ne!(agent_name, "");

    let agent_secret = env::var("AGENT_SECRET")
        .chain_err(|| "Missing environment variable: AGENT_SECRET")?;
    assert_ne!(agent_secret, "");

    let server_address = env::var("SERVER_ADDRESS")
        .chain_err(|| "Missing environment variable: SERVER_ADDRESS")?;
    assert_ne!(server_address, "");

    // Connect to the server
    let conn = TcpStream::connect(&server_address)?;
    let hs = Handshake{ version: backupd::VERSION, name: agent_name.clone(), secret: agent_secret };
    bincode::serialize_into(&conn, &hs)?;

    let _ack = read_ack(&conn)?;

    Ok(())
}

fn read_ack<R: Read>(s: R) -> e::Result<Ack> {
    Ok(bincode::deserialize_from(s)?)
}

fn main() {
    env_logger::init();
    if let Err(err) = client_start() {
        eprintln!("Error in client_start: {:?}", err);
    }
}

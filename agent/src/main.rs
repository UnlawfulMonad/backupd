use std::env;
use log::{info, error};

use std::net::TcpStream;

use backupd::error as e;
use backupd::error::ResultExt;
use backupd::protocol::{read_ack, Handshake};

/// The main client entrypoint. Starts the agent and the main loop.
///
/// The client pulls configuration from the following environment variables:
///   - `AGENT_NAME`
///   - `AGENT_SECRET`
///   - `SERVER_ADDRESS`
pub fn client_start() -> e::Result<()> {
    info!("Starting client...");

    // Load config
    let agent_name =
        env::var("AGENT_NAME").chain_err(|| "Missing environment variable: AGENT_NAME")?;
    assert_ne!(agent_name, "");

    let agent_secret =
        env::var("AGENT_SECRET").chain_err(|| "Missing environment variable: AGENT_SECRET")?;
    assert_ne!(agent_secret, "");

    let server_address =
        env::var("SERVER_ADDRESS").chain_err(|| "Missing environment variable: SERVER_ADDRESS")?;
    assert_ne!(server_address, "");

    // Connect to the server
    let conn = TcpStream::connect(&server_address)?;

    // Perform handshake
    let hs = Handshake {
        version: backupd::protocol::VERSION,
        name: agent_name.clone(),
        secret: agent_secret,
    };
    bincode::serialize_into(&conn, &hs)?;

    let ack = read_ack(&conn)?;
    if !ack.success {
        return Err(e::ErrorKind::HandshakeError(ack.message).into())
    }

    Ok(())
}

fn main() {
    env_logger::init();
    if let Err(err) = client_start() {
        error!("Error in client_start: {:?}", err);
    }
}

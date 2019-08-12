use std::env;
use log::{info, error};

use std::net::TcpStream;

use crate::error as e;
use crate::error::ResultExt;
use crate::protocol::{read_ack, Handshake};

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
        version: super::protocol::VERSION,
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

pub fn do_main() {
    if let Err(err) = client_start() {
        error!("Error in client_start: {:?}", err);
    }
}

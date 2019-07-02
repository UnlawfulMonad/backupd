use std::net::TcpStream;
use std::env;
use std::io::{self, Read};
use backupd::{Handshake, Ack};

fn client_start() -> io::Result<()> {
    let agent_name = env::var("AGENT_NAME").unwrap();
    assert_ne!(agent_name, "");

    let agent_secret = env::var("AGENT_SECRET").unwrap();
    assert_ne!(agent_secret, "");

    let server_address = env::var("SERVER_ADDRESS").unwrap();
    assert_ne!(server_address, "");

    let conn = TcpStream::connect(&server_address)?;
    let _hs = Handshake{ version: backupd::VERSION, name: agent_name.clone(), secret: agent_secret };

    let _ack = read_ack(&conn)?;

    Ok(())
}

fn read_ack<R: Read>(s: R) -> io::Result<Ack> {
    match bincode::deserialize_from(s) {
        Ok(ack) => Ok(ack),
        Err(err) => {
            let e = io::Error::new(io::ErrorKind::Other, err);
            Err(e)
        }
    }
}

fn main() {
    env_logger::init();
    if let Err(err) = client_start() {
        eprintln!("Error in client_start: {:?}", err);
    }
}

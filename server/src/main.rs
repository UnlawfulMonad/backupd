use std::io;
use std::fs::File;
use std::thread;
use std::net::{TcpListener, TcpStream};
use openssl::memcmp;

use backupd::Handshake;

use serde::Deserialize;
use log::{info, debug, trace};

use serde_json::from_reader;

/// Represents a backup agent.
#[derive(Clone, Debug, Deserialize)]
pub struct Agent {
    name: String,
    secret: String,
}

impl Agent {
    /// Get the name of the agent.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn secret_matches(&self, s: &str) -> bool {
        if self.secret.len() != s.len() {
            return false;
        }

        memcmp::eq(self.secret.as_bytes(), s.as_bytes())
    }
}

#[derive(Clone, Debug)]
struct Settings {
    agents: Vec<Agent>,
}

fn get_settings() -> io::Result<Settings> {
    let file = File::open("/etc/backupd/config.json")?;
    let agents = from_reader(file)?;

    Ok(Settings { agents, })
}

fn server_handler(stream: TcpStream) -> io::Result<()> {
    let _handshake: Handshake = rmp_serde::from_read(stream).expect("Failed to read handshake");

    Ok(())
}

fn main() {
    env_logger::init();

    info!("Loading settings...");
    let _settings = get_settings().expect("Unable to load settings");

    let listener = TcpListener::bind("").expect("Unable to bind");
    loop {
        let (stream, addr) = listener.accept().expect("Failed to accept connection...");
        debug!("Got connection from {:?}", addr);

        thread::spawn(move || {
            trace!("Connection info: TTL={}", stream.ttl().unwrap());
            if let Err(err) = server_handler(stream) {
                eprintln!("Got error from thread handler for {}: {:?}", addr, err);
            }
        });
    }
}

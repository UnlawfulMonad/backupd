use openssl::memcmp;
use std::env;
use std::fs::File;
use std::io;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::protocol::{read_handshake, write_ack, Ack};

use log::{debug, info, trace};
use serde::Deserialize;

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

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    agents: Vec<Agent>,
}

fn get_settings() -> io::Result<Settings> {
    let file_path = match env::var("CONFIG_FILE") {
        Ok(ref val) if val == "" => "./config.json".into(),
        Ok(val) => val,
        Err(env::VarError::NotPresent) => "./config.json".into(),
        Err(err) => panic!(err),
    };

    let file = File::open(&file_path)?;
    let settings = from_reader(file)?;

    Ok(settings)
}

pub fn server_handler(settings: &Settings, mut stream: TcpStream) -> io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;

    let handshake = read_handshake(&stream).expect("Failed to read handshake");

    // Find if we have an agent that corresponds to the sent credentials
    let agent = settings
        .agents
        .iter()
        .filter(|agent| {
            agent.get_name() == handshake.name && agent.secret_matches(&handshake.secret)
        })
        .next();

    // Respond
    write_ack(
        &mut stream,
        &Ack { success: agent.is_some(), message: None, }
    ).expect("Failed to write");

    Ok(())
}

pub fn do_main() {
    info!("Loading settings...");
    let settings = Arc::new(get_settings().expect("Unable to load settings"));

    let listener = TcpListener::bind("0.0.0.0:2345").expect("Unable to bind");
    loop {
        let (stream, addr) = listener.accept().expect("Failed to accept connection...");
        debug!("Got connection from {:?}", addr);

        let s = settings.clone();
        thread::spawn(move || {
            trace!("Connection info: TTL={}", stream.ttl().unwrap());
            if let Err(err) = server_handler(&s, stream) {
                eprintln!("Got error from thread handler for {}: {:?}", addr, err);
            }

            debug!("Closing connection to {:?}", addr);
        });
    }
}

use std::io;
use std::fs::File;

use serde::Deserialize;
use log::info;

use serde_json::from_reader;

#[derive(Clone, Debug, Deserialize)]
struct Agent {
    name: String,
    secret: String,
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

fn main() {
    env_logger::init();

    info!("Loading settings...");
    let _settings = get_settings().expect("Unable to load settings");
}

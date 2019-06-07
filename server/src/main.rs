use serde::Deserialize;
use std::env;
use log::info;

#[derive(Clone, Debug, Deserialize)]
struct Agent {
    name: String,
    secret: String,
}

#[derive(Clone, Debug)]
struct Settings {
    agents: Vec<Agent>,
}

fn get_settings() -> Settings {

    Settings {
        agents: Vec::new(),
    }
}

fn main() {
    env_logger::init();

    info!("Loading settings...");
    let _settings = get_settings();
}

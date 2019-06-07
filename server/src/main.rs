use std::env;
use log::info;

#[derive(Clone, Debug)]
enum AgentSource {
    File,
    Env,
}

#[derive(Clone, Debug)]
struct Agent {
    name: String,
    secret: String,
}

#[derive(Clone, Debug)]
struct Settings {
    agents: Vec<Agent>,
    agent_source: AgentSource,
}

fn get_settings() -> Settings {
    let agent_source = match env::var("") {
        Ok(var) => match &var as &str {
            "file" => AgentSource::File,
            "env" => AgentSource::Env,
            _ => panic!("invalid parameter for agent source"),
        }
        Err(_) => AgentSource::Env,
    };

    Settings {
        agent_source,
        agents: Vec::new(),
    }
}

fn main() {
    env_logger::init();

    info!("Loading settings...");
    let _settings = get_settings();
}

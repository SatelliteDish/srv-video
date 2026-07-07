/*
*   Config parsing logic for the daemon. The config hierarchy is:
*     1. JSON string passed directly
*     2. JSON file found at CONFIG_FILE
*     3. Environment variables
*   Each level overwrites those below with a DEFAULT_HOST and DEFAULT_PORT if either is missing
*/


use srv_host_core::get_config_dir;
use serde::Deserialize;
use std::{
    env,
    fs,
    io::Read,
};




const CONFIG_FILE: &str = "host.json";
const DEFAULT_HOST: &str = "http://127.0.0.1";
const DEFAULT_PORT: u16 = 8080;

#[derive(Debug)]
pub struct SrvHostConfig {
    address: String,
}

impl SrvHostConfig {
    pub fn address(&self) -> &str {
        &self.address
    }
}

impl Default for SrvHostConfig {
    fn default() -> Self {
        Self {
            address: format!("{DEFAULT_HOST}:{DEFAULT_PORT}"),
        }
    }
}


#[derive(Debug, Default, Deserialize)]
struct SrvHostConfigBuilder {
    address: Option<String>,
}


fn from_env(builder: &mut SrvHostConfigBuilder) {
    if let Ok(addr) = env::var("SRV_ADDRESS") && builder.address.is_none() {
        builder.address = Some(addr);
    }
}

fn from_config_file(builder: &mut SrvHostConfigBuilder) {
    let mut config_path = match get_config_dir() {
        Ok(dir) => dir,
        Err(e) => {
            println!("Could not check config dir:\n{e}");
            return;
        },
    };
    config_path.push(CONFIG_FILE);
    if fs::exists(&config_path).expect(&format!("Could not check if {} exists",config_path.to_string_lossy())) {
        let mut file = fs::File::open(config_path.as_path()).expect("Config file cannot be opened");
        let mut config_str = String::new();
        file.read_to_string(&mut config_str).expect("Could not read config file as String");
        let parsed: SrvHostConfigBuilder = serde_json::from_str(&config_str).expect("Could not parse config file as JSON");

        if let Some(addr) = parsed.address && builder.address.is_none() {
            builder.address = Some(addr);
        }
    }
}

fn from_config_str(builder: &mut SrvHostConfigBuilder, config_str: &str) {
    let parsed: SrvHostConfigBuilder = serde_json::from_str(config_str).expect("Could not parse config file as JSON");

    if let Some(addr) = parsed.address && builder.address.is_none() {
        builder.address = Some(addr);
    }
}

pub fn get_config(config_str: Option<&str>) -> SrvHostConfig {
    let mut builder = SrvHostConfigBuilder::default();
    if let Some(conf_str) = config_str {
        from_config_str(&mut builder, conf_str);
    }
    from_config_file(&mut builder);
    from_env(&mut builder);

    let def = SrvHostConfig::default();
    SrvHostConfig {
        address: builder.address.unwrap_or(def.address),
    }
}


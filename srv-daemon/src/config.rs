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




const CONFIG_FILE: &str = "daemon-conf.json";
const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 8080;

#[derive(Debug)]
pub struct SrvDaemonConfig {
    host: String,
    port: u16,
    feed_path: Option<String>, // URL Path override, default is same as file name
}

impl SrvDaemonConfig {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Default for SrvDaemonConfig {
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST.to_string(),
            port: DEFAULT_PORT,
            feed_path: None,
        }
    }
}


#[derive(Debug, Default, Deserialize)]
struct SrvDaemonConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    feed_path: Option<Option<String>>,
}


fn from_env(builder: &mut SrvDaemonConfigBuilder) {
    if let Ok(host) = env::var("SRV_HOST") && builder.host.is_none() {
        builder.host = Some(host);
    }
    if let Ok(port) = env::var("SRV_PORT") && builder.port.is_none() {
        builder.port = Some(port.parse::<u16>().expect("SRV_PORT could not be parsed"));
    }
    if let Ok(feed_path) = env::var("SRV_FEED_PATH") && builder.feed_path.is_none() {
        builder.feed_path = if feed_path.is_empty() { Some(None) } else { Some(Some(feed_path)) };
    }

}

fn from_config_file(builder: &mut SrvDaemonConfigBuilder) {
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
        let parsed: SrvDaemonConfigBuilder = serde_json::from_str(&config_str).expect("Could not parse config file as JSON");

        if let Some(host) = parsed.host && builder.host.is_none() {
            builder.host = Some(host);
        }
        if let Some(port) = parsed.port && builder.port.is_none() {
            builder.port = Some(port);
        }
        if let Some(feed_path) = parsed.feed_path && builder.feed_path.is_none() {
            builder.feed_path = Some(feed_path);
        }
    }
}

fn from_config_str(builder: &mut SrvDaemonConfigBuilder, config_str: &str) {
    let parsed: SrvDaemonConfigBuilder = serde_json::from_str(config_str).expect("Could not parse config file as JSON");

    if let Some(host) = parsed.host && builder.host.is_none() {
        builder.host = Some(host);
    }
    if let Some(port) = parsed.port && builder.port.is_none() {
        builder.port = Some(port);
    }
    if let Some(feed_path) = parsed.feed_path && builder.feed_path.is_none() {
        builder.feed_path = Some(feed_path);
    }
}

pub fn get_config(config_str: Option<&str>) -> SrvDaemonConfig {
    let mut builder = SrvDaemonConfigBuilder::default();
    if let Some(conf_str) = config_str {
        from_config_str(&mut builder, conf_str);
    }
    from_config_file(&mut builder);
    from_env(&mut builder);

    let def = SrvDaemonConfig::default();
    SrvDaemonConfig {
        host: builder.host.unwrap_or(def.host),
        port: builder.port.unwrap_or(def.port),
        feed_path: builder.feed_path.unwrap_or(def.feed_path),
    }
}

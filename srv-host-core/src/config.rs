use serde::Deserialize;
use std::{
    env,
    fs,
    io::Read as _,
};

use super::get_config_dir;

const CONFIG_FILE: &str = "host-conf.json";
const DEFAULT_FEED_FILE: &str = "feed.xml";

#[derive(Debug)]
pub struct SrvHostConfig {
    data_dir: Option<String>, // Override for default
    feed_file: String, // Feed file name
}

impl Default for SrvHostConfig {
    fn default() -> Self {
        Self {
            data_dir: None,
            feed_file: DEFAULT_FEED_FILE.to_string(),
        }
    }
}


#[derive(Debug,Default,Deserialize)]
struct SrvHostConfigBuilder {
    data_dir:Option<Option<String>>,
    feed_file: Option<String>,
}

fn from_env(builder: &mut SrvHostConfigBuilder) {
    if let Ok(data_dir) = env::var("SRV_DATA_DIR") && builder.data_dir.is_none() {
        builder.data_dir = if data_dir.is_empty() { Some(None) } else { Some(Some(data_dir)) };
    }
    if let Ok(feed_file) = env::var("SRV_FEED_FILE") && builder.feed_file.is_none() {
        builder.feed_file = Some(feed_file);
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

        if let Some(data_dir) = parsed.data_dir && builder.data_dir.is_none() {
            builder.data_dir = Some(data_dir);
        }
        if let Some(feed_file) = parsed.feed_file && builder.feed_file.is_none() {
            builder.feed_file = Some(feed_file);
        }
    }
}

fn from_config_str(builder: &mut SrvHostConfigBuilder, config_str: &str) {
    let parsed: SrvHostConfigBuilder = serde_json::from_str(config_str).expect("Could not parse config file as JSON");

    if let Some(data_dir) = parsed.data_dir && builder.data_dir.is_none() {
        builder.data_dir = Some(data_dir);
    }
    if let Some(feed_file) = parsed.feed_file && builder.feed_file.is_none() {
        builder.feed_file = Some(feed_file);
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
        data_dir: builder.data_dir.unwrap_or(def.data_dir),
        feed_file: builder.feed_file.unwrap_or(def.feed_file),
    }
}


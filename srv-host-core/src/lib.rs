use directories::ProjectDirs;
use thiserror::Error;
use std::{
    io,
    path::{
        PathBuf,
        Path,
    },
    result,
    fs,
};

pub mod database;
pub mod rss;
pub mod config;


const ORG_NAME: &str = "srv-video";
const APP_NAME: &str = "srv-host";



pub trait SrvError: std::error::Error {
    fn to_base(self) -> SrvHostError;
}



#[derive(Debug, Error)]
pub enum SrvHostError {
    #[error("{0}")]
    IoError(#[from]IoErr),
    #[error("An unexpected error occurred:\n{0}")]
    Unexpected(String),
}
pub type Result<T> = result::Result<T, SrvHostError>;

#[derive(Debug, Error)]
pub enum IoErr {
    #[error("An unexpected error occurred:\n{0}")]
    Unexpected(#[from]io::Error),
}

impl SrvError for IoErr {
    fn to_base(self) -> SrvHostError {
        self.into()
    }
}


fn get_proj_dir() -> result::Result<ProjectDirs, IoErr> {
    ProjectDirs::from("com", ORG_NAME, APP_NAME)
        .ok_or(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Could not determine HOME directory"
        ).into())
}

fn ensure_dir(dir: &Path) -> result::Result<&Path, IoErr> {
    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }

    Ok(dir)
}

pub fn get_data_dir() -> result::Result<PathBuf, IoErr> {
    let proj_dir = get_proj_dir()?;
    let dir = &proj_dir.data_dir();

    ensure_dir(dir)
        .map(|d| d.to_path_buf())
}

pub fn get_config_dir() -> result::Result<PathBuf,IoErr> {
    let proj_dir = get_proj_dir()?;
    let dir = &proj_dir.config_dir();

    ensure_dir(dir)
        .map(|d| d.to_path_buf())
}

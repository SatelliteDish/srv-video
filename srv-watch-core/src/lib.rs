use directories::ProjectDirs;
use std::{
    fs,
    path::{
        PathBuf,
        Path,
    },
};

mod config;
pub mod database;

pub mod feed;
pub mod error;

fn get_proj_dir() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "srv-video", "srv-watch")
}

fn ensure_dir(dir: &Path) -> &Path {
    if !dir.exists() {
        fs::create_dir_all(dir).unwrap();
    }

    dir
}

pub fn get_data_dir() -> Option<PathBuf> {
    let proj_dir = get_proj_dir()?;
    let dir = &proj_dir.data_dir();

    Some(ensure_dir(dir).to_path_buf())
}

pub fn get_cache_dir() -> Option<PathBuf> {
    let proj_dir = get_proj_dir()?;
    let dir = &proj_dir.cache_dir();

    Some(ensure_dir(dir).to_path_buf())
}

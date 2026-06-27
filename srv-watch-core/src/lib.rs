use directories::ProjectDirs;
use std::{
    fs,
    path::PathBuf,
};

mod config;
pub mod database;

pub mod feed;
pub mod error;

pub fn get_work_dir() -> Option<PathBuf> {
    ProjectDirs::from("com", "srv-video", "srv-watch")
        .map(|dir| {
            if let Ok(false) = fs::exists(dir.data_dir()) {
                fs::create_dir_all(dir.data_dir()).unwrap();
            }
            dir.data_dir().to_path_buf()
        })
}

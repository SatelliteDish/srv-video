use rusqlite::Connection;
use thiserror::Error;

use super::get_work_dir;

pub mod follow;


#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("An unexpected error occurred:\n{0}")]
    UnexpectedError(String),
}

impl From<rusqlite::Error> for DatabaseError {
    fn from(value: rusqlite::Error) -> Self {
        Self::UnexpectedError(value.to_string())
    }
}

impl From<reqwest::Error> for DatabaseError {
    fn from(value: reqwest::Error) -> Self {
        Self::UnexpectedError(value.to_string())
    }
}

impl From<rss::Error> for DatabaseError {
    fn from(value: rss::Error) -> Self {
        Self::UnexpectedError(value.to_string())
    }
}

pub fn get_db_connection() -> Connection {
    let mut dir = get_work_dir()
        .unwrap();
    dir.push("sql.db");
    let con = Connection::open(dir.as_path());

    println!("{con:?}");

    con.unwrap()
}

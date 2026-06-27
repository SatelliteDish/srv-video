use rusqlite::Connection;
use thiserror::Error;
use std::{
    result,
};

use crate::{
    get_data_dir,
    SrvHostError,
    IoErr,
    SrvError,
};

pub mod videos;


#[derive(Debug, Error)]
pub enum SrvHostDbErr {
    #[error("{0}")]
    IoErr(#[from]IoErr),
    #[error("{0}")]
    DbErr(#[from]rusqlite::Error),
}
pub type Result<T> = result::Result<T, SrvHostDbErr>;

impl SrvError for SrvHostDbErr {
    fn to_base(self) -> SrvHostError {
        match self {
            Self::IoErr(e) =>  e.into(),
            Self::DbErr(e) => SrvHostError::Unexpected(e.to_string()),
        }
    }
}


pub fn get_db_connection() -> Result<Connection> {
    let mut dir = get_data_dir()?;
    dir.push("db.sql");

    Connection::open(dir.as_path())
        .map_err(SrvHostDbErr::from)
}

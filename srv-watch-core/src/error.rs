use thiserror::Error;


#[derive(Debug, Error)]
pub enum WatchError {
    #[error("Database Error: {0}")]
    DataBaseError(String)
}

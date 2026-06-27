use rusqlite::Connection;
use thiserror::Error;
use const_format::concatcp;
use nanoid::nanoid;
use std::{
    result,
};


const TABLE_NAME: &str = "videos";



#[derive(Debug, Error)]
pub enum DbVideoErr {
    #[error("{0}")]
    DatabaseErr(#[from]rusqlite::Error),
}
type Result<T> = result::Result<T, DbVideoErr>;


#[derive(Debug)]
pub struct VideoEntry {
    pub id: String,
    pub uri: String,
}

fn ensure_video_table(db: &Connection) -> Result<()> {
    if !db.table_exists(None, TABLE_NAME)? {
        db.execute(concatcp![
            "CREATE TABLE IF NOT EXISTS ",TABLE_NAME,"(id TEXT PRIMARY KEY, uri TEXT);"
        ],[])?;
    }

    Ok(())
}

pub async fn register_video(db: &Connection, path: &str) -> Result<String> {
    ensure_video_table(db)?;
    let id = nanoid!(10);

    db.execute(concatcp![
        "INSERT INTO ",TABLE_NAME,"(id, uri) VALUES (?1,?2);"
    ],[&id, path])?;

    Ok(id)
}


pub async fn query_videos(db: &Connection) -> Result<Vec<VideoEntry>> {
    ensure_video_table(db)?;

    db.prepare(concatcp!["SELECT id, uri FROM ",TABLE_NAME])
        .map_err(DbVideoErr::from)?
        .query_map([], |v| Ok(VideoEntry {
            id: v.get(0)?,
            uri: v.get(1)?,
        }))?
        .collect::<result::Result<Vec<VideoEntry>,_>>()
        .map_err(DbVideoErr::from)
}

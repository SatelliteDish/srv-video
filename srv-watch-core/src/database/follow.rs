use rusqlite::Connection;
use thiserror::Error;
use const_format::concatcp;
use srv_core::feed::Feed;

use crate::database::DatabaseError;

const TABLE_NAME: &str = "following";

#[derive(Debug, Error)]
pub enum DatabaseSubscriptionError {
    #[error("Query Error")]
    QueryError,
    #[error("{0}")]
    DatabaseError(#[from]DatabaseError),
}
type Result<T> = std::result::Result<T, DatabaseSubscriptionError>;

fn subscription_table_exists(db: &Connection) -> Result<bool> {
    Ok(db.table_exists(None, TABLE_NAME).map_err(DatabaseError::from)?)
}

fn ensure_subscription_table(db: &Connection) -> Result<()> {
    if subscription_table_exists(db)? {
        db.execute(concatcp![
            "CREATE TABLE IF NOT EXISTS ",TABLE_NAME,
            "(url TEXT PRIMARY KEY, title TEXT);"
        ],[]).map_err(DatabaseError::from)?;
    }

    Ok(())
}


#[derive(Debug)]
pub struct Subscription {
    pub url: String,
    pub feed: Feed,
}

pub async fn follow(db: &Connection, url: &str, feed: &Feed) -> Result<()> {
    ensure_subscription_table(db)?;

    db.execute(concatcp![
        "INSERT INTO ",TABLE_NAME,"(url, title)",
        "VALUES (?1,?2);"
    ], [&url, feed.title()])
        .map_err(DatabaseError::from)?;

    Ok(())
}


pub fn query_following(db: &Connection) -> Result<Vec<Subscription>> {
    ensure_subscription_table(db)?;

    db.prepare(concatcp!["SELECT url, title FROM ",TABLE_NAME])
        .map_err(DatabaseError::from)?
        .query_map([], |sub| Ok(Subscription {
            url: sub.get(0)?,
            feed: Feed {
                title: sub.get(1)?,
                ..Default::default()
            },
        }))
        .map_err(DatabaseError::from)? // Handle query error
        .collect::<std::result::Result<Vec<Subscription>,_>>() // Vec<Result> to Result<Vec>
        .map_err(|e| DatabaseError::from(e).into()) // Covert to expected err
}

pub fn delete_from_following(db: &Connection, url: &str) -> Result<()> {
    if !subscription_table_exists(db)? {
        return Err(DatabaseSubscriptionError::QueryError);
    }
    db.execute(concatcp![
        "DELETE from ",TABLE_NAME," WHERE url=?1;"
    ], [url])
        .map_err(DatabaseError::from)?;

    Ok(())
}

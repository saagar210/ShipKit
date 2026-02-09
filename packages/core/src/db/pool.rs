//! SQLite connection pool with WAL mode and foreign keys enabled by default.

use std::fmt;
use std::path::Path;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::error::Result;

/// Applies WAL mode and foreign keys on every new connection.
#[derive(Debug)]
struct PragmaCustomizer;

impl r2d2::CustomizeConnection<rusqlite::Connection, rusqlite::Error> for PragmaCustomizer {
    fn on_acquire(
        &self,
        conn: &mut rusqlite::Connection,
    ) -> std::result::Result<(), rusqlite::Error> {
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        Ok(())
    }
}

/// A thread-safe SQLite connection pool.
///
/// Enables WAL mode (concurrent reads) and foreign keys on every connection.
/// Safe to store in Tauri managed state without additional wrapping.
#[derive(Clone)]
pub struct ConnectionPool {
    pool: Pool<SqliteConnectionManager>,
}

impl fmt::Debug for ConnectionPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConnectionPool")
            .field("state", &self.pool.state())
            .finish()
    }
}

impl ConnectionPool {
    /// Open or create a SQLite database at the given path.
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::builder()
            .connection_customizer(Box::new(PragmaCustomizer))
            .build(manager)?;
        Ok(Self { pool })
    }

    /// Create a shared in-memory database (useful for tests).
    pub fn in_memory() -> Result<Self> {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::builder()
            .connection_customizer(Box::new(PragmaCustomizer))
            .max_size(1) // single connection to share in-memory state
            .build(manager)?;
        Ok(Self { pool })
    }

    /// Get a connection from the pool.
    pub fn get(&self) -> Result<r2d2::PooledConnection<SqliteConnectionManager>> {
        Ok(self.pool.get()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_creation_and_query() {
        let pool = ConnectionPool::in_memory().expect("pool creation");
        let conn = pool.get().expect("get connection");
        conn.execute_batch("CREATE TABLE t (id INTEGER PRIMARY KEY)")
            .expect("create table");
    }

    #[test]
    fn wal_mode_enabled() {
        // In-memory databases may report "memory" instead of "wal", so test with a file.
        let tmp = tempfile::TempDir::new().expect("tmp dir");
        let file_pool = ConnectionPool::new(tmp.path().join("test.db")).expect("pool creation");
        let file_conn = file_pool.get().expect("get connection");
        let mode: String = file_conn
            .query_row("PRAGMA journal_mode", [], |row| row.get(0))
            .expect("query journal_mode");
        assert_eq!(mode, "wal");
    }

    #[test]
    fn foreign_keys_enabled() {
        let pool = ConnectionPool::in_memory().expect("pool creation");
        let conn = pool.get().expect("get connection");
        let fk: i32 = conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .expect("query foreign_keys");
        assert_eq!(fk, 1);
    }
}

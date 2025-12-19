use rusqlite::Connection;
use std::error;
use thiserror::Error;

use crate::npm;

const SCHEMA_REGISTRY_V1_0_0: &str = include_str!("schema_registry_v1.0.0.sql");

#[derive(Error, Debug)]
pub enum FindAllError {
    #[error("Failed to execute SQL query: {0}")]
    SelectError(#[from] rusqlite::Error),
    #[error("Failed to retrieve registry URL: {0}")]
    RegistryError(#[from] Box<dyn error::Error>),
}

pub struct DatabaseManager {
    connection: Connection,
}

#[derive(Debug)]
pub struct Registry {
    pub name: String,
    pub url: String,
    pub is_current: bool,
}

impl DatabaseManager {
    pub fn init() -> Result<Self, rusqlite::Error> {
        let connection = Connection::open("npm-registry.db")?;
        connection.execute_batch(SCHEMA_REGISTRY_V1_0_0)?;
        Ok(Self { connection })
    }

    pub fn find_all(&self) -> Result<Vec<Registry>, FindAllError> {
        let current_registry = npm::config::get_registry()?;
        self.update_current(&current_registry)?;
        let mut stmt = self
            .connection
            .prepare("SELECT name, url, is_current FROM registry")?;
        let registry_vec: Vec<Registry> = stmt
            .query_map([], |row| {
                Ok(Registry {
                    name: row.get(0)?,
                    url: row.get(1)?,
                    is_current: row.get(2)?,
                })
            })?
            .collect::<Result<_, _>>()?;
        Ok(registry_vec)
    }

    pub fn find_url_by_name(&self, name: &str) -> Result<String, rusqlite::Error> {
        let mut stmt = self
            .connection
            .prepare("SELECT url FROM registry WHERE name = ?")?;
        let url = stmt.query_row([name], |row| row.get(0))?;
        Ok(url)
    }

    pub fn update_current(&self, current_registry: &str) -> Result<(), rusqlite::Error> {
        self.connection
            .execute("UPDATE registry SET is_current = 0", ())?;
        self.connection.execute(
            "UPDATE registry SET is_current = 1 WHERE url = ?",
            [current_registry],
        )?;
        Ok(())
    }
}

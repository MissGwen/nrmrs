use sqlite::State;
use std::{collections::HashMap, error};

const INIT_SQL: &str = include_str!("schema_registry_v1.0.0.sql");

pub struct DatabaseManager {
    connection: sqlite::Connection,
}

impl DatabaseManager {
    pub fn init() -> Result<Self, Box<dyn error::Error>> {
        let connection = sqlite::open("registry.db")?;
        connection.execute(INIT_SQL)?;
        Ok(Self { connection })
    }

    // pub fn create_registry(&self, name: &str, url: &str) -> Result<(), Box<dyn error::Error>> {
    //     let insert_sql = format!(
    //         "INSERT INTO registry (name, url) VALUES ('{}', '{}')",
    //         name, url
    //     );
    //     self.connection.execute(&insert_sql)?;
    //     Ok(())
    // }

    pub fn find_all(&self) -> Result<HashMap<String, String>, Box<dyn error::Error>> {
        let query = "SELECT name, url, is_current FROM registry";
        let mut map: HashMap<String, String> = HashMap::new();
        let mut statement = self.connection.prepare(query)?;
        while let Ok(State::Row) = statement.next() {
            // let is_current = statement.read::<i64, _>("is_current")?;
            // println!("is_current: {}", is_current);
            map.insert(
                statement.read::<String, _>("name")?,
                statement.read::<String, _>("url")?,
            );
        }
        Ok(map)
    }
}

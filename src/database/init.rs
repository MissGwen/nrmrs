use sqlite::State;
use std::{collections::HashMap, env, error, fs};

const SQL_FILE_NAME: &str = "init.sql";

pub struct DatabaseManager {
    connection: sqlite::Connection,
}

impl DatabaseManager {
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        let connection = sqlite::open("registry.db")?;
        let current_dir = env::current_dir()?;
        let sql_path = current_dir.join("src").join("database").join(SQL_FILE_NAME);
        let query = fs::read_to_string(sql_path)?;
        connection.execute(&query)?;
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
        let query = "SELECT name, url FROM registry";
        let mut map: HashMap<String, String> = HashMap::new();
        let mut statement = self.connection.prepare(query)?;
        while let Ok(State::Row) = statement.next() {
            map.insert(
                statement.read::<String, _>("name")?,
                statement.read::<String, _>("url")?,
            );
        }
        Ok(map)
    }
}

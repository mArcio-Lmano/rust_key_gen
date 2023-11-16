// use druid::widget::prelude::*;
// use druid::widget::{Flex, Label, TextBox};
// use druid::{AppLauncher, Data, PlatformError, Widget, WidgetExt, WindowDesc, Lens, UnitPoint};
use druid::{Data, Lens};
use rusqlite::{Connection, Result};

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::fmt;


struct ChatSet{
    char_set: Vec<char>
}
impl ChatSet {
    fn new() -> ChatSet{
        ChatSet { 
            char_set: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{}|;:'\",.<>/?"
            .chars()
            .collect() 
        }
    }
    fn change() -> ChatSet{
        let char_set = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{}|;:'\",.<>/?"
            .chars()
            .collect();
        ChatSet { 
            char_set 
        }
    }
}

fn generate_password(length: usize, charset: Vec<char>) -> String {

    let mut rng = thread_rng();
    let password: String = (0..length)
        .map(|_| *charset.choose(&mut rng).unwrap())
        .collect();

    password
}

#[derive(Clone, Data, Lens, Debug)]
pub struct PassSave {
    pub id: i32,
    pub site: String,
    pub user: String,
    pub pass: String,
}

impl PassSave{
    pub fn new() -> Self{
        PassSave{
            id: 0,
            site: "".to_string(),
            user: "".to_string(),
            pass: "".to_string(),
        }
    }
}


impl fmt::Display for PassSave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Define how PassSave should be formatted when using {}
        write!(f, "Site: {}, User: {}, PassSave: {}", self.site, self.user, self.pass)
    }
}

impl PartialEq for PassSave {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.site == other.site && self.user == other.user && self.pass == other.pass
    }
}

impl Eq for PassSave {}

pub struct Database {
    connection: Connection,
}


impl Database {
    // Constructor method to create a new instance of Database
    pub fn new(database_path: &str) -> Result<Self> {
        let connection = Connection::open(database_path)?;
        Ok(Database { connection })
    }

    // Create table if it doesn't exist
    pub fn create_table(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS passsave (
                  id INTEGER PRIMARY KEY,
                  site TEXT NOT NULL,
                  user TEXT NOT NULL,
                  pass TEXT NOT NULL)",
            [],
        )?;
        Ok(())
    }

    // Insert a password entry into the database
    pub fn insert_passsave(&self, site: &str, user: &str, pass: &str) -> Result<()> {
        self.connection.execute(
            "INSERT INTO passsave (site, user, pass) VALUES (?1, ?2, ?3)",
            &[site, user, pass],
        )?;
        Ok(())
    }

    // Query all password entries from the database
    pub fn query_all_passsaves(&self) -> Result<Vec<PassSave>> {
        let mut stmt = self.connection.prepare("SELECT id, site, user, pass FROM passsave")?;
        let passsave_iter = stmt.query_map([], |row| {
            Ok(PassSave {
                id: row.get(0)?,
                site: row.get(1)?,
                user: row.get(2)?,
                pass: row.get(3)?,
            })
        })?;

        let mut passsaves = Vec::new();
        for passsave in passsave_iter {
            passsaves.push(passsave?);
        }

        Ok(passsaves)
    }
}



// fn main() -> Result<(), PlatformError> {

//     let database = Database::new("dubg.db")
//     .expect("Failed to Create a DataBase struct");


//     // Create a table if it doesn't exist
//     let _ = database.create_table()
//     .expect("Failed to create or open the table");

//     // Insert some data
//     let _ = database.insert_passsave("Facebook", "Jonh White", "test_password")
//     .expect("Failed to Add Entery");

//     // Query the data
//     let passsaves = database.query_all_passsaves()
//     .expect("Failed to get the enteries");

//     for passsave in passsaves {
//         println!("Found Entery {}", passsave);
//     }

//     Ok(())
// }

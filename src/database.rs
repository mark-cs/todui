use std::{
    self,
    fs::{self, File, OpenOptions},
    io::{BufReader, BufWriter, Error},
    path::Path,
};

use crate::todo::Todo;

pub struct Database {
    path: String,
    pub todos: Vec<Todo>,
}

impl Database {
    const DB_PATH: &str = ".local/share/todui/db.json";
    pub fn new(path: String) -> Database {
        Database {
            path,
            todos: Vec::new(),
        }
    }

    pub fn read(&mut self) -> Result<(), Error> {
        if !fs::exists(&self.path)? {
            Database::init_new_db(&self.path)?
        }

        // read the contents of the database
        let mut db_todos: Vec<Todo> =
            serde_json::from_reader(BufReader::new(File::open(&self.path)?))?;

        self.todos.append(&mut db_todos);

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        let writer = BufWriter::new(OpenOptions::new().write(true).open(&self.path)?);
        serde_json::to_writer(writer, &self.todos)?;

        Ok(())
    }

    pub fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    fn init_new_db(path: &str) -> Result<(), Error> {
        // database does not exist, create an empty database
        fs::create_dir_all(
            Path::new(path)
                .parent()
                .and_then(|p| p.to_str())
                .ok_or(Error::other("Unable to locate database file's parent"))?,
        )?;
        fs::write(path, "[]")?;
        Ok(())
    }

    pub fn default_db_path() -> Result<String, Error> {
        match dirs::home_dir().map(|h| h.join(Database::DB_PATH)) {
            Some(path) => path
                .to_str()
                .map(|s| s.to_owned())
                .ok_or(Error::other("ohh dear")),
            None => Err(Error::other("ohh dear")),
        }
    }
}

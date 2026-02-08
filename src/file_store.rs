use std::{
    self,
    fs::{self, File, OpenOptions},
    io::{BufReader, BufWriter, Error},
    path::Path,
};

use crate::store::Store;
use crate::todo::Todo;

/// A store that persits todos to file
pub struct FileStore {
    /// The location of the file that contains the persisted todos
    path: String,
}

impl FileStore {
    /// Default location of the file containing the todos
    const DB_PATH: &str = ".local/share/todui/db.json";
    /// Create a new FileStore, storing the todos in a file at a given path
    pub fn new(path: String) -> FileStore {
        FileStore { path }
    }

    /// Crteate a new blank todo file
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

    /// Get the path to the default location
    pub fn default_db_path() -> Result<String, Error> {
        match dirs::home_dir().map(|h| h.join(FileStore::DB_PATH)) {
            Some(path) => path
                .to_str()
                .map(|s| s.to_owned())
                .ok_or(Error::other("ohh dear")),
            None => Err(Error::other("ohh dear")),
        }
    }
}
impl Store for FileStore {
    fn read(&self) -> Result<Vec<Todo>, Error> {
        if !fs::exists(&self.path)? {
            FileStore::init_new_db(&self.path)?
        }

        let db_todos: Vec<Todo> = serde_json::from_reader(BufReader::new(File::open(&self.path)?))?;

        Ok(db_todos)
    }
    fn write(&self, todos: &[Todo]) -> Result<(), Error> {
        let writer = BufWriter::new(OpenOptions::new().write(true).open(&self.path)?);
        serde_json::to_writer(writer, todos)?;

        Ok(())
    }
}

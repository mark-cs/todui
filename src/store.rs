use crate::todo::Todo;
use std::io::Error;

/// A trait that allows todos to be read and written
pub trait Store {
    /// Read all the todos from the store
    fn read(&self) -> Result<Vec<Todo>, Error>;

    /// Replace the store todos with the ones provided
    fn write(&self, todo: &[Todo]) -> Result<(), Error>;
}

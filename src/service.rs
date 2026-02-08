use std::io::Error;

use crate::store::Store;
use crate::todo::Todo;

/// Provides functions to interact with a store of todos
pub struct TodoService<T: Store> {
    /// The underlying store of todos
    store: T,
}

impl<T: Store> TodoService<T> {
    /// Create a TodoService using an underlying store
    pub fn new(store: T) -> TodoService<T> {
        TodoService { store }
    }

    /// Add a todo to the stored todos
    pub fn add_todo(&mut self, todo: Todo) -> Result<(), Error> {
        let mut todos = self.store.read()?;
        todos.push(todo);
        self.store.write(&todos)?;

        Ok(())
    }

    /// Get a reference to the list of stored todos
    pub fn all(&self) -> Result<Vec<Todo>, Error> {
        self.store.read()
    }

    /// Get a reference to the list of uncompleted stored todos
    pub fn only_uncompleted(&self) -> Result<Vec<Todo>, Error> {
        Ok(self
            .store
            .read()?
            .into_iter()
            .filter(|t| !t.completed)
            .collect())
    }
}

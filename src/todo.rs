use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub description: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(description: String) -> Todo {
        Todo {
            description,
            completed: false,
        }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Description: {}, Completed: {}",
            self.description, self.completed
        )
    }
}

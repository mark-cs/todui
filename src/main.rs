mod cli;
mod database;
mod todo;

use std::io::Error;

use clap::Parser;
use cli::Cli;
use database::Database;

use crate::{cli::Commands, todo::Todo};

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let mut database = Database::new(Database::default_db_path()?);
    database.read().expect("Unable to read database file");

    match &cli.command {
        Some(Commands::Add { description }) => {
            database.add_todo(Todo::new(description.to_owned()));
            database.flush()?;
        }
        Some(Commands::List) => {
            for todo in database.todos.into_iter() {
                println!(
                    "{} {}",
                    if todo.completed { "[x]" } else { "[ ]" },
                    todo.description
                );
            }
        }
        None => {}
    }

    Ok(())
}

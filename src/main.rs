mod cli;
mod file_store;
mod service;
mod store;
mod todo;

use std::io::Error;

use clap::Parser;
use cli::Cli;
use service::TodoService;

use crate::{cli::Commands, file_store::FileStore, todo::Todo};

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let store = FileStore::new(FileStore::default_db_path()?);

    let mut service = TodoService::new(store);

    match &cli.command {
        Some(Commands::Add { description }) => {
            service.add_todo(Todo::new(description.to_owned()))?
        }
        Some(Commands::List { all }) => {
            let todos = match *all {
                true => service.all()?,
                false => service.only_uncompleted()?,
            };
            for todo in todos.iter() {
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

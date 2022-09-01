use clap::{Args, Parser, Subcommand};
use crate::todo::*;
use crate::todolist::*;

use std::path::PathBuf;


pub fn parse_args(cli: PleaseArgs, todo_file: &PathBuf){
    let mut todo_list = read_todo_file(todo_file);

    match cli.command {
        Commands::Showtasks(s) => {
            showtasks(s, &todo_list);
            return
        }

        Commands::Add(a) => {
            let text = a.text;
            let priority = a.priority.unwrap_or(0);
            let class = a.class.unwrap_or(String::from(""));
            let todo = ToDo::new(text, priority, class);
            add_todo(todo, &mut todo_list);
        },
        Commands::Remove(r) => {
            let index = r.index;
            match remove_todo(index, &mut todo_list) {
                Ok(()) => {},
                Err(_) => println!("Could not remove {:?}", index)
            }
        },
        Commands::Change(c) => {
            let index = c.index;
            if index >= todo_list.len() {
                println!("Could not change {:?}", index);
            }
            let todo = &mut todo_list[index];
            todo.change(c.text, c.priority, c.class)        },
        Commands::Do(d) => {
            let index = d.index;
            if index >= todo_list.len() {
                println!("Could not mark {:?} as done", index);
            }
            let todo = &mut todo_list[index];
            todo.mark_done()
        },
        Commands::Undo(u) => {
            let index = u.index;
            if index >= todo_list.len() {
                println!("Could not mark {:?} as not done", index);
            }
            let todo = &mut todo_list[index];
            todo.mark_undone()        },
        Commands::Clear => {
            todo_list.retain(|todo| !todo.is_done())        }
    };

    save_todo_list(&todo_list, todo_file);
    showtasks(Showtasks { priority: false, class: false }, &todo_list)
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PleaseArgs {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    Showtasks(Showtasks),
    Add(AddToDo),
    Remove(Index),
    Do(Index),
    Undo(Index),
    Change(Change),
    Clear
}

#[derive(Debug, Args)]
pub struct Showtasks {
    #[clap(short, long, action)]
    pub priority: bool,

    #[clap(short, long, action)]
    pub class: bool
}

#[derive(Debug, Args)]
struct AddToDo {
    /// the actual todo
    text: String,
    
    /// priority of the todo
    #[clap(short, long)]
    priority: Option<usize>,

    /// class of todo
    #[clap(short, long)]
    class: Option<String>
}

#[derive(Debug, Args)]
struct RemoveToDo {
    /// ID of todo to be removed
    todo_id: usize
}

#[derive(Debug, Args)]
struct Index{
    /// Index of todo
    index: usize
}

#[derive(Debug, Args)]
struct Change {
    index: usize,

    #[clap(short, long)]
    text: Option<String>,

    #[clap(short, long)]
    priority: Option<usize>,

    #[clap(short, long)]
    class: Option<String>
}


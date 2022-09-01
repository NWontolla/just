mod args;
mod todo;
mod todolist;

use std::{fs, path::PathBuf};

use clap::Parser;
use args::{PleaseArgs, parse_args};

const SAVEDIR: &str = ".just";
const SAVEFILE: &str = "todo_list.txt";

fn main(){
    let todo_file = setup();
    let cli = PleaseArgs::parse();
    parse_args(cli, &todo_file);
}

fn setup() -> PathBuf{
    // Does some setup, like creating the directories and save files.

    // Get the home directory
    let home_dir = match home::home_dir() {
        Some(dir) => dir,
        None => panic!("Cannot find home directory") // TODO: implement get directory
    };

    // Create the save directory if it does not exist.
    let save_dir = home_dir.join(SAVEDIR);
    if !std::path::Path::exists(&save_dir) {
        fs::create_dir(&save_dir).unwrap();
    }

    // Create save file if it does not exists.
    let todo_file = save_dir.join(SAVEFILE);
    if !std::path::Path::exists(&todo_file){
        fs::File::create(&todo_file).unwrap();
    }

    return todo_file;
}


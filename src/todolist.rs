use crate::todo::{self, ToDo, ToDoStatus};
use crate::args;
use std::fs::{self, File};
use std::path::PathBuf;
use std::io::Write;
use term_table::table_cell::TableCell;
use term_table::{self, TableStyle};


pub enum Errors{
    IndexError
}

pub fn read_todo_file(file_name: &PathBuf) -> Vec<todo::ToDo> {
    fs::read_to_string(file_name)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| ToDo::from_string(line).unwrap())
        .collect()
}

pub fn save_todo_list(todo_list: &Vec<todo::ToDo>, file_name: &PathBuf) {
    let mut file = File::create(file_name).unwrap();

    for todo in todo_list {
        let s = todo.to_string();
        writeln!(&mut file, "{}", s).unwrap();
    }
}

pub fn add_todo(todo: ToDo,
                todo_list: &mut Vec<todo::ToDo>){
    todo_list.push(todo);
}

pub fn remove_todo(index: usize, todo_list: &mut Vec<todo::ToDo>) -> Result<(), Errors>{
    if index >= todo_list.len() {
        return Err(Errors::IndexError)
    } 
    todo_list.remove(index);
    Ok(())
}

pub fn showtasks(arg: args::Showtasks, todo_list: &Vec<todo::ToDo>) {
    let mut indices = (0..todo_list.len()).collect::<Vec<_>>();
    
    if arg.priority {
        indices.sort_by_key(|&i| &todo_list[i].priority);
    }
    if arg.class {
        indices.sort_by_key(|&i| &todo_list[i].class);
    }

    let mut table = term_table::Table::new();
    table.max_column_width = 80;
    table.style = TableStyle::extended();

    table.add_row(term_table::row::Row::new(vec![
        TableCell::new("Index"),
        TableCell::new("Task"), 
        TableCell::new("Priority"), 
        TableCell::new("Class"), 
        TableCell::new("Status")
    ]));


    for (index, &i) in indices.iter().enumerate() {
        let todo_item = &todo_list[i];
        let status = match todo_item.status {
            ToDoStatus::Done => 'v',
            ToDoStatus::NotDone => 'x'
        };
        
        table.add_row(term_table::row::Row::new(vec![
            TableCell::new(index),
            TableCell::new(&todo_item.text), 
            TableCell::new(&todo_item.priority), 
            TableCell::new(&todo_item.class), 
            TableCell::new(status)
        ]));
    }

    println!("{}", table.render());
}

#[cfg(test)]
mod tests {
    use crate::todo::{self, ToDo};

    use super::*;

    #[test]
    fn test_add_todo_to_empty_list(){
        let mut todo_list: Vec<todo::ToDo> = vec![];
        let todo = ToDo::new(
            String::from("test"),
            1,
            String::from("test_class")
        );

        add_todo(todo, &mut todo_list);

        let todo = ToDo::new(
            String::from("test"), 
            1, 
            String::from("test_class")
        );

        assert_eq!(todo_list, vec![todo]);
    }

    #[test]
    fn test_add_todo_to_nonempty_list(){

        let mut todo_list: Vec<ToDo>= Vec::new();
        let first_todo = ToDo::new(
            String::from("first"), 
            1, 
            String::from("first_class")
        );

        add_todo(first_todo, &mut todo_list);

        let second_todo = ToDo::new(
            String::from("second"), 
            2, 
            String::from("second_class")
        );

        add_todo(second_todo, &mut todo_list);

        let supposed_todo_list = vec![
            ToDo::new(
                String::from("first"), 
                1, 
                String::from("first_class")
            ),
            ToDo::new(
                String::from("second"), 
                2, 
                String::from("second_class")
            )
        ];
        assert_eq!(todo_list, supposed_todo_list);
    }

    #[test]
    fn test_remove_todo(){
        let first_todo = ToDo::new(String::from("first"),1 , String::from("first"));
        let second_todo = ToDo::new(String::from("second"), 2, String::from("second"));
        let third_todo = ToDo::new(String::from("third"), 3, String::from("third"));

        let first_todo_copy = ToDo::new(String::from("first"),1 , String::from("first"));
        let third_todo_copy = ToDo::new(String::from("third"), 3, String::from("third"));

        let mut todo_list: Vec<ToDo> = Vec::new();

        add_todo(first_todo, &mut todo_list);
        add_todo(second_todo, &mut todo_list);
        add_todo(third_todo, &mut todo_list);
        remove_todo(1, &mut todo_list);

        assert_eq!(todo_list, vec![first_todo_copy, third_todo_copy]);
    }

}
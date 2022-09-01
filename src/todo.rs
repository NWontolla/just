use std::num::ParseIntError;

#[derive(Debug, PartialEq, Clone)]
pub enum ToDoStatus {
    Done,
    NotDone
}

#[derive(Debug, PartialEq, Clone)]
pub struct ToDo {
    pub text: String,
    pub priority: usize,
    pub class: String,
    pub status: ToDoStatus
}

impl ToDo {
    pub fn new(text: String, priority: usize, class: String) -> Self{
        ToDo { text, priority, class, status: ToDoStatus::NotDone }
    }

    pub fn is_done(&self) -> bool{
        self.status == ToDoStatus::Done
    }

    pub fn change(&mut self, 
              text: Option<String>, 
              priority: Option<usize>, 
              class: Option<String>){
        if let Some(t) = text {
            self.text = t;
        }
        if let Some(p) = priority {
            self.priority = p;
        }
        if let Some(c) = class {
            self.class = c;
        }
    }

    pub fn mark_done(&mut self){
        self.status = ToDoStatus::Done
    }

    pub fn mark_undone(&mut self){
        self.status = ToDoStatus::NotDone
    }

    pub fn from_string(s: &str) -> Result<Self, ParseIntError> {
        let splits: Vec<_> = s.split(',').collect();

        let priority = match splits[1].parse::<usize>(){
            Ok(p) => p,
            Err(e) => return Err(e)
        };
        
        let status = if splits[3] == "0" {ToDoStatus::NotDone} else {ToDoStatus::Done};
        Ok(Self{
            text: String::from(splits[0]), 
            priority, 
            class: String::from(splits[2]),
            status}
        )
    }

    pub fn to_string(&self) -> String {
        let status: u8 = if self.is_done() {1} else {0};
        format!("{},{},{},{}", self.text, self.priority, self.class, status)
    }
}


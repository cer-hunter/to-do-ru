use colored::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::Read;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;
use std::{env, process};

pub struct Todoru {
    pub todo: Vec<String>,
    pub todo_path: String,
    pub todo_bak: String,
    pub no_backup: bool,
}

impl Todoru{
    pub fn new() -> Result<Self, String>{
        let todo_path: String = match env::var("TODO_PATH") {
            Ok(t) => t,
            Err(_) => {
                let home = env::var("HOME").unwrap();
                // Look for a legacy TODO file path
                let legacy_todo = format!("{}/TODO", &home);
                match Path::new(&legacy_todo).exists() {
                    true => legacy_todo,
                    false => format!("{}/.todo", &home),
                }
            }
        };
        let todo_bak: String = match env::var("TODO_BAK_DIR"){
            Ok(t) => t,
            Err(_) => String::from("tmp/todo.bak"),
        };

        let no_backup = env::var("TODO_NOBACKUP").is_ok();

        let todofile = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(&todo_path)
        .expect("Couldn't open the todofile");

    // Create new buf reader
    let mut buf_reader = BufReader::new(&todofile);
    //Empty String ready to be filled with TODO
    let mut contents = String::new();
    // Load "contents" with string data
    buf_reader.read_to_string(&mut contents).unwrap();
    //Splits contents of the TODO file into a TODO vector
    let todo = contents.lines().map(str::to_string).collect();
    //Returns todo
    Ok(Self {
        todo,
        todo_path,
        todo_bak,
        no_backup,
    })
    }
    //List each todo item
    pub fn list (&self) { 
        let stdout = io::stdout();
        //buffered writer for stdout stream
        let mut writer = BufWriter::new(stdout);
        let mut data = String::new();
        //This loop will repeat for each task of the TODO file
        for (number, item) in self.todo.iter().enumerate() {
            if item.len() > 4{
                let number = (number + 1).to_string().bold();
                //saves symbol of current task
                let symbol = &item[..4];
                //saves task without a symbol
                let item = &item[4..]; 
                //Check if complete
                if symbol == "[x]"{
                    // DONE
                    //if task completed print it with a strikethru
                    data = format!("{} {}\n", number, item.strikethrough());
                } else if symbol == "[ ]"{
                    //NOT DONE
                    // print as is
                    data = format!("{} {}\n", number, item);
                }
            }
            writer
                .write_all(data.as_bytes())
                .expect("Failed to write to stdout");
        }
    }
    pub fn add(&self, args: &[String]){
        if args.is_empty(){
            eprintln!("Todoru add takes at least 1 argument");
            process::exit(1);
        }
        // Open TODO file with permissions
        let todofile = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&self.todo_path)
        .expect("Couldn't open the To-do-ru file");

        let mut buffer = BufWriter::new(todofile);
        for arg in args {
            if arg.trim().is_empty(){
                continue;
            }
            //Append new task
            let line = format!("[ ] {}\n", arg);
            buffer
                .write_all(line.as_bytes())
                .expect("unable to write data");
        }
    }
}
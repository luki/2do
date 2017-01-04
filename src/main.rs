extern crate chrono;

use std::io::prelude::*; // (Write_all)
use std::fs::File;
use std::error::Error; // (why.description...)
use chrono::*; // Date stuff

// TODO:
/*
    - Check for item
    - Implement date for creation & checked, etc
*/

// MODELLING

struct Item {
    content: String
} impl Item {
    fn new(content: &str) -> Item {
        Item { content: content.to_string() }
    }
}

struct List {
    name: String,
    creation_date: DateTime<UTC>,
    items: Vec<Item>
} impl List {
    fn new(name: &str, creation_date: DateTime<UTC>, items: Vec<Item>) -> List {
        List {
            name: name.to_string(),
            creation_date: creation_date,
            items: items
        }
    }
    fn create(&self) {
        let mut file = create_file(&self.name, "txt");
        let mut formatted_string = format!("{}\n{}\n\n", self.name, self.creation_date);

        for item in &self.items {
            formatted_string.push_str(&format!("*   {}\n", item.content));
        }

        write_to_file(&mut file, &formatted_string);
    }
}

// BASIC METHODS

fn create_file(file_name: &str, file_extension: &str) -> File {
    let mut file = match File::create(format!("{}.{}", file_name, file_extension)) {
        Ok(file) => file,
        Err(why) => panic!("An error occured: {}", why.description())
    };
    file
}

fn open_file(file_name: &str, file_extension: &str) -> File {
    let file = match File::open(format!("{}.{}", file_name, file_extension)) {
        Ok(file) => file,
        Err(why) => panic!("An error occured: {}", why.description())
    };
    file
}

fn read_from_file(file: &mut File) -> String {
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(content) => content,
        Err(why) => panic!("Could not read from file. Error: {}", why.description())
    };
    content
}

fn write_to_file(file: &mut File, content: &str) {
    // READ FROM FILE RIGHT THERE
    file.write_all(content.as_bytes());
}

fn override_file(file: &mut File, content: &str) {
    file.write_all(content.as_bytes());
}

// MAIN

fn main() {
    let list = List::new("example", UTC::now(), vec![
        Item::new("Some information"),
        Item::new("Test"),
        Item::new("Holly")
    ]);
    list.create();
    let utc: DateTime<UTC> = UTC::now();
    // let xyz = List::from_file("example", "txt");
}

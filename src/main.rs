extern crate chrono;

use std::io::prelude::*; // (Write_all)
use std::fs::File;
use std::error::Error; // (why.description...)
use chrono::*; // Date stuff

/* -------------------------------
            MODELLING
--------------------------------*/

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
    fn save_as_file(&self) {
        let mut file = create_file(&self.name, "txt");
        let mut formatted_string = format!("{}\n\n{}\n\n", self.name, self.creation_date);

        for item in &self.items {
            formatted_string.push_str(&format!("*\t{}\n", item.content));
        }

        write_to_file(&mut file, &formatted_string);
    }
    fn from_file(file: &mut File) -> List {
        let mut content = read_from_file(file);
        let mut content_vec: Vec<&str> = content.split("\n\n").collect();

        // As UTC formatting is wrong, removing all after . (milliseconds)

        let date_vec: Vec<&str> = content_vec[1].split("").collect();
        let mut temp_date_vec: Vec<&str> = Vec::new();

        for i in 0..date_vec.len() {
            if date_vec[i] == "." {
                break
            }
            temp_date_vec.push(date_vec[i]);
        }

        let date = match UTC.datetime_from_str(&format!("{}", temp_date_vec.join("")), "%Y-%m-%d %H:%M:%S") {
            Ok(date) => date,
            Err(why) => panic!("{}", why)
        };
        println!("{:?}", content_vec[content_vec.len()-1]);
        let items_text_vec: Vec<&str> = content_vec[content_vec.len()-1].split("*").collect();
        let mut item_vec: Vec<Item> = Vec::new();

        for item in &items_text_vec {
            item_vec.push(
                Item::new(&item)
            )
        }

        List::new(content_vec[0], date, item_vec)
    }
}

/* -------------------------------
        BASIC I/O METHODS
--------------------------------*/

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
    // let list = List::new("school", UTC::now(), vec![
    //     Item::new("Do math homework"),
    //     Item::new("Study for German test"),
    //     Item::new("Revise history")
    // ]);
    //
    // list.save_as_file();
    let list = List::from_file(&mut open_file("school", "txt"));

    println!("Title: {}\nDate: {}\n", list.name, list.creation_date);
    for item in &list.items {
        println!("{:?}", item.content)
    }
}

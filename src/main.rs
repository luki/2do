use std::fs::File;
use std::error::Error;

// MODELLING

struct Item {
    content: String
} impl Item {
    fn new(content: String) -> Item {
        Item { content: content }
    }
}

struct List {
    name: String,
    items: Vec<Item>
} impl List {
    fn new(name: &str, items: Vec<Item>) -> List {
        List {
            name: name.to_string(),
            items: items
        }
    }
    fn save(&self) {
        create_file(&self.name, "txt")
    }
}

// BASIC METHODS

fn create_file(file_name: &str, file_extension: &str) {
    let file = match File::create(format!("{}.{}", file_name, file_extension)) {
        Ok(file) => file,
        Err(why) => panic!("An error occured: {}", why.description())
    };
}

fn open_file(file_name: &str, file_extension: &str) {
    let file = match File::open(format!("{}.{}", file_name, file_extension)) {
        Ok(file) => file,
        Err(why) => panic!("An error occured: {}", why.description())
    };
}

fn main() {
    let list = List::new("example", vec![]);
    list.save();
}

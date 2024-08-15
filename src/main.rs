use std::io;

struct TodoItem {
    id: u64,
    title: String,
    completed: bool,
}

struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }

    fn remove_item_by_title(&mut self, title_to_remove: &str) -> Option<TodoItem> {
        if let Some(index) = self.items.iter().position(|item| item.title == title_to_remove) {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    fn add_item(&mut self, title: String) {
        let id = (self.items.len() as u64) + 1;
        let new_item = TodoItem {
            id,
            title: title.clone(),
            completed: false,
        };
        self.items.push(new_item);
        println!("Added: {}", title);
    }

    fn list_items(&self) {
        if self.items.is_empty() {
            println!("Your to-do list is empty.");
        } else {
            println!("Your to-do list:");
            for item in &self.items {
                let status = if item.completed { "[X]" } else { "[ ]" };
                println!("{} {} - {}", status, item.id, item.title);
            }
        }
    }

    fn complete_item(&mut self, id: u64) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.completed = true;
            println!("Completed: {}", item.title);
        } else {
            println!("Item with ID {} not found.", id);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        println!("1. Add Item");
        println!("2. List Items");
        println!("3. Complete Item");
        println!("4. Remove an Item with title");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter the title of the new item:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                todo_list.add_item(title.trim().to_string());
            }
            2 => {
                todo_list.list_items();
            }
            3 => {
                println!("Enter the ID of the completed item:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u64 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        continue;
                    }
                };
                todo_list.complete_item(id);
            }
            5 => {
                println!("Exiting the program.");
                break;
            }
            4 => {
                println!("Enter the title of the note you want to delete.");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");

                
                let removed_item_option = todo_list.remove_item_by_title(title.trim());

                if let Some(removed_item) = removed_item_option {
                    println!("Item with title '{}' has been removed", removed_item.title);
                } else {
                    println!("No item found with the title '{}'", title);
                }
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 5.");
            }
        }
    }
}

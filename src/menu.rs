use std::io;
use std::io::*;

pub struct Menu {
    pub options: Vec<MenuOption>
}

impl Menu {
    pub fn new(options: Vec<MenuOption>) -> Menu {
        Menu { options: options }
    }

    pub fn print(&self) {
        for i in 0..self.options.len() {
            println!("{0}. {1}", i + 1, self.options[i].description);
        }
        print!("> ");
        io::stdout().flush().expect("Unable to flush stdout");
    }

    pub fn get_user_choice(&self) -> &MenuOption {
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option_as_int: usize = option.trim().parse().expect("Please select one of the valid options");
        return &self.options[option_as_int - 1];
    }
}

pub struct MenuOption {
    pub description: String,
    pub tag: MenuTag
}

#[derive(PartialEq, Copy, Clone)]
pub enum MenuTag {
    MultiplesOf3And5,
    EvenFibonacciNumbers,
    Quit
}

mod menu;
mod problem;
mod problems;

use crate::menu::*;
use crate::problem::*;
use crate::problems::multiples_of_3_and_5::*;

fn main() {
    // TODO: clear terminal output at start
    let options: Vec<MenuOption> = vec!(
        MenuOption { description: String::from("Multiples of 3 and 5"), tag: MenuTag::MultiplesOf3And5 },
        MenuOption { description: String::from("Quit"), tag: MenuTag::Quit }
    );
    let menu = Menu::new(options);

    loop {
        println!("\n>>>>>>>>>>>>>> Welcome to the Euler project in rust! <<<<<<<<<<<<<<");
        println!("Pick a problem:");
        menu.print();
        let option = menu.get_user_choice();
        if option.tag == MenuTag::Quit {
            break;
        } else if option.tag == MenuTag::MultiplesOf3And5 {
            MultiplesOf3And5::new().execute(); 
        }
    }
}

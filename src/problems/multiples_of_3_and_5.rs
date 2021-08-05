use std::io;
use std::io::*;
use crate::problem::*;

#[derive(PartialEq)]
pub struct MultiplesOf3And5 {
    pub name: String,
}

impl MultiplesOf3And5 {
    pub fn new() -> MultiplesOf3And5 {
        MultiplesOf3And5 {
            name: String::from("Multiples of 3 and 5")
        }
    }
}

impl Executable for MultiplesOf3And5 {
    fn execute(&self) {
        println!("\n>>>>>>>>>>>>>>>> {0}", &self.name);
        println!("First, give us a number:");
        print!("> ");
        io::stdout().flush().expect("Unable to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read given input");
        
        let number: usize = input.trim().parse().expect("Unable to parse given input as an integer");
        
        let mut sum: usize = 0;
        for n in 1..number {
            let multiple_of_3 = n % 3 == 0;
            let multiple_of_5 = n % 5 == 0;

            if multiple_of_3 || multiple_of_5 {
                sum += n;
            }
        }

        println!("The sum of multiples of 3 and 5 up to {0} is: {1}", number, sum);
    }
}

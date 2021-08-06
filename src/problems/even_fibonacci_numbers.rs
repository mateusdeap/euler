use crate::problem::*;

pub struct EvenFibonacciNumbers {
    pub name: String,
}

impl EvenFibonacciNumbers {
    pub fn new() -> EvenFibonacciNumbers {
        EvenFibonacciNumbers {
            name: String::from("Even Fibonacci Numbers")
        }
    }
}

impl Executable for EvenFibonacciNumbers {
    fn execute(&self) {
        println!("Solve problem!")
    }
}

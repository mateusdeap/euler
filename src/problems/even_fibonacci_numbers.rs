use std::io;
use std::io::*;
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
    
    pub fn sum_of_even_fibonaccis_up_to(&self, ceiling: usize) -> usize {
        let mut sum: usize = 0;
        let fibonacci_sequence: Vec<usize> = self.fibonacci_sequence_up_to(ceiling);
        for num in fibonacci_sequence {
            if num % 2 == 0 {
                sum += num;
            }
        }
        
        return sum;
    }
    
    pub fn fibonacci_sequence_up_to(&self, ceiling: usize) -> Vec<usize> {
        let mut fibonacci_seq: Vec<usize> = Vec::new();
        let mut prev_fib: usize = 0;
        let mut next_fib: usize = 1;
        
        while next_fib < ceiling {
            fibonacci_seq.push(next_fib);
            let tmp = prev_fib;
            prev_fib = next_fib;
            next_fib = next_fib + tmp;
        }
        
        return fibonacci_seq;
    }
}

impl Executable for EvenFibonacciNumbers {
    fn execute(&self) {
        println!("First, we need a number to be the ceiling of our Fibonacci sequence");
        print!("> ");
        io::stdout().flush().expect("Unable to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read given input");
        
        let ceiling: usize = input.trim().parse().expect("Unable to parse given input as an integer");
        let sum = self.sum_of_even_fibonaccis_up_to(ceiling);
        println!("The sum of all even Fibonacci numbers up to {0} is: {1}", ceiling, sum);
    }
}

extern crate fizzbuzz;
use std::io;

fn main() {

    println!("Please enter a number:");
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            let number = input.trim().parse::<i32>().ok().unwrap();
            println!("{}", fizzbuzz::fizz_buzz(number));
        },
        Err(error) => println!("{}", error)
    }

    /*for i in 1..101 {
        println!("{}", fizzbuzz::fizz_buzz(i));
    }*/
}

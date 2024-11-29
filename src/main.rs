

use std::{io};

fn main() {

    let mut count = 0;
    let mut fn2 = 0; //Fibonacci Number Fn-2
    let mut fn1 = 1; //Fibonacci Number Fn-1
    let mut fn_sum = 0; //Fibonacci Number

    loop{
        println!("Which sequence of n would you like to use for your Fibonacci number?");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let sequence = n.trim().parse::<i32>().expect("Please type a number!");

        while count < sequence-1 {
            if sequence == 0 {
                fn_sum = fn2;
            } else if sequence == 1 {
                fn_sum = fn1;
            } else {

                fn_sum = fn2 + fn1; //Get the current Fibonacci Number

                fn2 = fn1;

                fn1 = fn_sum;

                count = count + 1;

            }
        }
        println!("The fibonacci number is {}", fn_sum);
    }


}

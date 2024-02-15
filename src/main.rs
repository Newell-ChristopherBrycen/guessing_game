//  -------------------------------------------------------------------------------
// | Name:          Brycen Newell                                                  |
// | Date:          2/14/2024                                                      |
// | Project:       guessing_game                                                  |
// | Description:   The game should request user input                             |
// |                and compare it to the values of a random secret number         |
// |                if correct the program should exit and congratulate the user   |
// | Last Update: 2/14/2024 CBN                                                    |
//  -------------------------------------------------------------------------------

//use statements bring types into the scope.
//it is necessary to add these scopes to the standard library to allow for them to be used

use std::io; // for input output
use rand::Rng; // for random number generator
use std::cmp::Ordering; // for comparison

fn main() {

    println!("Guess the number!");

    // create the secret_number here
    let secret_number = rand::thread_rng().gen_range(1..=100); // range format

    // println!("The secret number is: {secret_number}"); THIS LINE WAS FOR TESTING



    // this loop iterates without end until meeting an exit requirement
    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        // verifies variable type matches the expected "String" type
        io::stdin()
            .read_line(&mut guess)
            // & indicates a reference to the mutable variable
            // this allows for the use of the same variable without
            // having to read that same variable into memory again
            .expect("Failed to read line");
        // .expect allows us to handle a potential error
        // this puts whatever the read line string value into
        // a "Result" enum that gives a final value of 'Ok' or 'err'
        // Result types have an expect method that determines
        // how to behave if the Result is 'Err'

        // this updates the var type to address the Ordering error in the match statement. not used in final due to better error checking existing
        // let guess: u32 = guess.trim().parse().expect("Please type a number!"); //this parses the value into an unsigned (positive) 32-bit number

        // this is a good way of error checking in which we perform the same parse, but we meet the
        // Result types of Ok and Err with two different behaviors.
        // if we achieve the expected result, which is an u32 value, then we export guess with an u32 for the next portion of the program
        // otherwise, with an error we start a new iteration of the loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");  // this is useful, {variable} is a placeholder for that variable

        match guess.cmp(&secret_number) { // there is an error based on type since Ordering looks for an int but sees a string
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

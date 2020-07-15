use std::io;
use std::cmp::Ordering;
use rand::Rng;
// The Rng trait defines methods that random number generators implement, and this
// trait must be in scope for us to use those methods.

fn main() {
    println!("Guess the number!");

    // The rand::thread_rng function will give us the particular random number generator
    // that we’re going to use: one that is local to the current thread of execution
    // and seeded by the operating system.
    // Then we call the gen_range method on the random number generator
    // This method is defined by the Rng trait that we brought into scope with the use rand::Rng statement.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Infinite loop. Yes, that's it.
    loop {
        println!("Please input your guess.");

        // variables are immutable by default so we need to add the mut keyword
        // because we want to modify this variable
        let mut guess = String::new(); // new is like a "static" method of the String type
        // we can access "static" methods using ::

        // stdin() creates a handler to the standard input of the current process.
        // read_line() takes one string argument and it stores
        // the value entered by the user and stores it to given argument
        // this argument variable must be mutable, and also we add the & keyword to
        // make it works a reference value.
        // References are immutable by default, that's why we
        // write &mut guess rather than &guess to make it mutable.
        // read_line() returns a Result type, Results are like Enums
        // for Result, te variants are Ok or Err.
        // we use the expect function (from Result types) to do something depending on the result value.
        // If this instance of io::Result is an Err value, expect will cause the program to
        // crash and display the message that you passed as an argument to expect.
        // If this instance of io::Result is an Ok value, expect will take the return value that Ok
        // is holding and return just that value to you so you can use it. In this case, that value
        // is the number of bytes in what the user entered into standard input.
        // note: we are not storing the return value from read_line
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Hey, we already have a guess variable. WTF?
        // Rust allows us to shadow the previous value of guess with a new one.
        // first, we trim the string because we don't want the \n value coming from the enter key
        // then we parse the string into some kind of number.
        // Because the method parse can "parse" a variety of number types, we need to tell Rust the exact
        // number type we want by using let guess: u32.
        // The colon (:) after guess tells Rust we’ll annotate the variable’s type
        // We use the keyword match to handle a possible error thrown by the parse method
        // If parse is able to successfully turn the string into a number, it will return an Ok value
        // that contains the resulting number. That Ok value will match the first arm’s pattern,
        // and the match expression will just return the num value that parse produced and put inside
        // the Ok value. That number will end up right where we want it in the new guess variable we’re creating.
        // If parse is not able to turn the string into a number, it will return an Err value that
        // contains more information about the error. The Err value does not match the Ok(num)
        // pattern in the first match arm, but it does match the Err(_) pattern in the second arm.
        // The underscore, _, is a catchall value; in this example, we’re saying we want to match all
        // Err values, no matter what information they have inside them. So the program will execute
        // the second arm’s code, continue, which tells the program to go to the next iteration of
        // the loop and ask for another guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue // _ is a catchall value, we want to match all Err values
        };

        println!("You guessed: {}", guess);

        // The cmp method compares two values and can be called on anything that can be compared
        // It takes a reference to whatever you want to compare with:
        // here it’s comparing the guess to the secret_number.
        // Then it returns a variant of the Ordering enum we brought into scope with the use statement.
        // We use a match expression to decide what to do next based on which variant of Ordering was
        // returned from the call to cmp with the values in guess and secret_number.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // we break the infinite loop, whoohoooo
            }
        };
    }
}

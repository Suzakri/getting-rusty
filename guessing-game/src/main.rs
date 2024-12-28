use std::io; // library imports/"crates"
use std::cmp::Ordering;
use rand::Rng; // Defines RNG traits

// No function types
fn main() {
    // 1-10 inclusive
    let secret = rand::thread_rng().gen_range(1..=10);

    loop {
    println!("Guess a number, go ahead!\nMake your guess!");

    // Opt-in mutability
    // Javascript-esque variable declaration: let/const(?)
    let mut guess = String::new(); // Unicode dynamically sized string

    // User console input with built-in error checking
    io::stdin()
      // References are immutable by default, thus &mut to opt in to mutability
      .read_line(&mut guess) // Passing arg as a "reference"
      .expect("Failed to read number");

    // match statements are kind of fascinating actually.
    // Basically, you're handling control flow on patterns
    // (often enums) being matched. Sort of like switch statements
    // except they have the ability to handle wildcards for more
    // advanced control

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    match guess.cmp(&secret) {
      Ordering::Less => println!("Your guess was too small!"),
      Ordering::Greater => println!("Your guess was too big!"),
      Ordering::Equal => {
        println!("Bingo!");
        break;
      }
    }
  }
}

use std::cmp::Ordering;
use std::io;
use rand::Rng;

/// # Guess the Number
///
/// This program generates a secret number between 1 and 100,
/// and asks the user to guess the number. It gives feedback on
/// whether the guess is too small, too high, or equal to the
/// secret number until the user guesses the correct number.
fn main() {
  println!("Guess the number!");
  let secret_number = rand::thread_rng().gen_range(1..=100);
  println!("The secret number is {}", secret_number);
  loop {
    println!("Please input your guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("Your guess is: {}", guess);
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small"),
      Ordering::Greater => println!("Too high"),
      Ordering::Equal => {
        println!("You win");
        break;
      }
    }
  }
}

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number game:");
    loop {
    println!("Please input your Number.");
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=4);
    io::stdin()
        .read_line(&mut guess)
            .expect("Failed to read line");

    println!("Secret Number: {secret_number}");
    println!("You guessed: {guess}");

    let guess:u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small!");
        },
        Ordering::Greater => {
            println!("Too big!");
        },
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
    }
}

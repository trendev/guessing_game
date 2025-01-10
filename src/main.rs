use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let n: u32 = 100;
    let max = n.ilog2() + 1;

    let secret_number = rand::thread_rng().gen_range(1..=n);
    let mut success = false;

    for i in (1..=max).rev() {
        println!("{i} - Please input your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("<< Too small!"),
            Ordering::Greater => println!(">> Too big!"),
            Ordering::Equal => {
                println!("## You WIN! ##");
                success = true;
                break;
            }
        }
    }

    if !success {
        println!("## Sorry, you LOOSE :( ##");
    }
}

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let n: u32 = 100;
    let max = n.ilog2() + 1;
    println!("Guess the number in {max} attempts!");

    let secret_number = get_secret_number(n);
    let mut success = false;

    for i in (1..=max).rev() {
        println!("({i}) - Please input your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!(">> Too small!"),
            Ordering::Greater => println!("<< Too big!"),
            Ordering::Equal => {
                println!("## You WIN! ##");
                success = true;
                break;
            }
        }
    }

    if !success {
        println!("## Sorry, you LOOSE :( secret number was {secret_number} ##");
    }
}

fn get_secret_number(n: u32) -> u32 {
    rand::thread_rng().gen_range(1..=n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_number_between_1_and_100() {
        let result = get_secret_number(100);
        assert!(
            result >= 1 && result <= 100,
            "result must be between 1 and 100"
        );
    }
}

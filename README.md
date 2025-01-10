# Guessing Game

A simple Rust program where the player tries to guess a randomly generated number between 1 and 100.

## How It Works

1. The program generates a random secret number between 1 and 100.
2. The player has a maximum of `log2(100) + 1 = 7` attempts to guess the number.
3. After each guess, the program provides feedback:
   - **<< Too small!**: The guess is lower than the secret number.
   - **>> Too big!**: The guess is higher than the secret number.
   - **## You WIN! ##**: The guess matches the secret number, and the game ends.
4. If the player fails to guess the number within the allowed attempts, the game ends with a **"Sorry, you LOOSE :("** message.

## How to Run

1. Clone this repository:
```shell
git clone https://github.com/trendev/guessing_game.git
cd guessing_game
```


2. Build and run the program:
```shell
cargo run
```


3. Follow the prompts to input your guesses.

## Example Output

### Winning Example


## Requirements

- Rust (version `1.67` or later)

Enjoy playing and improving your guessing skills (using binary search logic or dichotomy...)!

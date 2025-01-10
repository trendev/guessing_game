# Guessing Game

A simple Rust program where the player tries to guess a randomly generated number between 1 and 100.

## How It Works

1. The program generates a random secret number between 1 and 100.
2. The player has a maximum of `log2(100) + 1 = 7` attempts to guess the number.
3. After each guess, the program provides feedback:
   - **>> Too small!**: The guess is lower than the secret number.
   - **>> Too big!**: The guess is higher than the secret number.
   - **## You WIN! ##**: The guess matches the secret number, and the game ends.
4. If the player fails to guess the number within the allowed attempts, the game ends with a **"Sorry, you LOOSE :("** message.

## How to Run

### 1. Clone this repository

```shell
git clone https://github.com/trendev/guessing_game.git
cd guessing_game
```

### 2. Build and run the program

```shell
cargo run
```

### 3. Follow the prompts to input your guesses

## Example Output

### Winning Example

```cmd
Guess the number in 7 attempts!
7 - Please input your guess:
50
You guessed: 50
<< Too big!
6 - Please input your guess:
25
You guessed: 25
>> Too small!
5 - Please input your guess:
37
You guessed: 37
>> Too small!
4 - Please input your guess:
42
You guessed: 42
## You WIN! ##
```

### Losing Example

```cmd
Guess the number in 7 attempts!
7 - Please input your guess:
50
You guessed: 50
<< Too big!
6 - Please input your guess:
25
You guessed: 25
>> Too small!
5 - Please input your guess:
37
You guessed: 37
>> Too small!
4 - Please input your guess:
40
You guessed: 40
>> Too small!
3 - Please input your guess:
45
You guessed: 45
<< Too big!
2 - Please input your guess:
43
You guessed: 43
<< Too big!
1 - Please input your guess:
41
You guessed: 41
>> Too small!
## Sorry, you LOOSE :( ##
```

## Requirements

- Rust (version `1.67` or later)

> Enjoy playing and improving your guessing skills (using binary search logic or dichotomy...)!

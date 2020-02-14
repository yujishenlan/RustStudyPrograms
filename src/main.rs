fn main() {
    hello_world();
    guessing_game();
}

fn hello_world(){
    println!("Hello，Rust world!");
}

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn guessing_game() {
    // 猜数字游戏。
    // 在最大猜测次数内，根据提示猜测给定的数字。

    const MAX_GUESS_TIME: u32 = 10; //最大猜测次数
    println!("=================================================");
    println!("      Welcome to the Quessing Game Program.      ");
    println!("Guess the number!");

    // 随机产生秘密数字
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // 已猜测次数
    let mut guess_time: u32 = 0;
    while guess_time < MAX_GUESS_TIME {
        println!("Please input your guess.");
        // 输入猜测数字
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                guess_time += 1;
                num
            }
            Err(_) => continue,
        };
        println!("Round {}: You guessed: {}.", guess_time, guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("You have guessed {} time!", guess_time);
    if guess_time >= MAX_GUESS_TIME {
        println!("The secret number is: {}.", secret_number);
        println!("You lose!");
    }
    println!("================== GameOver! ====================");
}

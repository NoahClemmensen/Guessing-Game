use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut guessing = true;
    let secret = gen_range(1, 10);

    while guessing {
        clear();
        println!("Guess the number!");
        let guess: i32 = get_input_with_text("Please type a number").trim().parse().expect("Please type a number!");

        match guess.cmp(&secret) {
            Ordering::Less => {
                println!("You guessed too low, guess again!\n");
            },
            Ordering::Greater => {
                println!("You guessed too high, guess again!\n");
            },
            Ordering::Equal => {
                guessing = false;
                println!("You guessed correct! The secret number was {}", secret);
            }
        }
        get_input_with_text("");
    }

    println!("The secret number was {}", secret);
}

fn get_input_with_text(str: &str) -> String {
    println!("{}", str);

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input;
}

fn gen_range(i: i32, k: i32) -> i32 {
    return rand::thread_rng().gen_range(i..k);
}

fn clear() {
    print!("{}[2J", 27 as char);
}
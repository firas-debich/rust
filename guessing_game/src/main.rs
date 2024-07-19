use std::{ cmp::Ordering, io };
use colored::Colorize;
use rand::Rng;

fn main() {
    println!(" Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret Number is :{}", secret_number);

    loop {
        println!("Please input you guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line ");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
        println!(" You gussed : {} ", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","To Big!".red()),
            Ordering::Equal => {
                println!("{}","You Win !".green());
                break;
            }
        }
    }
}
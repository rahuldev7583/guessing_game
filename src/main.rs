use std::io;
use rand::Rng; 
use std::cmp::Ordering;

fn main() {
    println!("Welcome to Guess a number game!");


    let secret_num = rand::thread_rng().gen_range(0..=100);
    // println!("The secret num is {}", secret_num);

    loop{
        println!("Enter a number");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read");

    

    // let guess : u32 = guess.trim().parse().expect("Please enter a number");
    let guess : u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
    println!("You guessed {}", guess);

    //campare via if else
    // if guess == secret_num {
    //     println!("Congrats! You guessed right")
    // } else if guess < secret_num {
    //     println!("Better luck next time! The secret number was bigger {}", secret_num)
    // } else{
    //     println!("Better luck next time! The secret number was smaller {}", secret_num)
    // }

    // campre via cmp Ordering method using match
    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Better luck next time! Number was bigger"),
        Ordering::Greater => println!("Better luck next time! Number was smaller"),
        Ordering::Equal => {
            println!("Congrats! You guessed right");
            break;
        }
    }

    }
}

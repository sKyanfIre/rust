use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is:{}",secret_number);

    loop{
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("fail to read number");
        let guess:i32 = guess.trim().parse().expect("Please type a number");
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Less!"),
            Ordering::Greater => println!("Too Greater"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("you guessed: {}",guess);
    }

}

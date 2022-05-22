use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    
    // You can do 1..=100 as well as a range from 1 to 100
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut allowed_tries = 5;

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("============================");
        println!("You guessed: {}", guess);

        

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"), 
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
       
        allowed_tries-=1;
        
        println!("Tries left: {}", allowed_tries);
        println!("============================");

        if allowed_tries == 0 {
            println!("No more tries left. You lose!");
            break;
        }
    }
}

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop{
        println!("GUESS THE NUMBER!");

        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("RANDOM NUMBER IS {secret_number}");
        println!("Please input your guess");
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_)  => {
                println!("\t\t***THE NUMBER PASSED IS NOT VALID***");
                continue;
            }
        };
        println!("YOU GUESSED: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less =>{
                println!("Too small!");
            }
            Ordering::Greater => {
                println!("Too big!");
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }   
    }
    
}

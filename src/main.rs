use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and 100...");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;
    
    loop{
		println!("\nPlease input your guess:");
		
		let mut guess = String::new();
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");
			
		let guess: u32 = match guess.trim().parse(){
			Ok(num) => num,
			Err(_) => {
				println!("Please enter a valid number!");
				continue;
				}
			};
			attempts +=1;
			
			match guess.cmp(&secret_number){
				Ordering::Less => println!("Too small! Try a bigger number."),
				Ordering::Greater => println!("Too ig! Try a smaller number"),
				Ordering::Equal => {
					println!("Congratulations! you guessed it in {} attempts!", attempts);
					println!("The number was {}!", secret_number);
					break;
					}
				}
		}
}

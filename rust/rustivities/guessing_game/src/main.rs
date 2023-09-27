use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

   // println!("The secret number is: {}", secret_number);

    loop {
        
        println!("Please Input Your Guess: ");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read_line");
    
        //let guess: u32 = guess.trim().parse().expect("Please Type in a number"); 
        //move from expect call to a match expression to move from crashng on an error to handling
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        
        }
    }
    

}

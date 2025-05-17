use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the Number");
    
    let secret_number  = rand::rng().random_range(1..= 100);
    
    loop {
        println!("Please input your guess");
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        println!("your guessed:{guess}");


        // let guess : u32 = guess
        //     .trim()
        //     .parse()
        //     .expect("Please type a number");
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match  guess.cmp(&secret_number){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
        }
    }
}

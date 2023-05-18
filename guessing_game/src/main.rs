use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    
    let randomnum = rand::thread_rng().gen_range(2, 53);
    loop{
        println!("guess the number!");
       
        println!("please enter the number you guessed: ");
    
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");
        // let guess:u32 = guess.trim().parse().expect("shuda entered a number!");
        let guess:u32 = match  guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,

        };
        println!("you guessed {}", guess); 
    
        match guess.cmp(&randomnum){
           Ordering::Equal =>{println!("{}","yay! you are right!".green());
                              break;} ,
           Ordering::Less => println!("{}","its something larger than that!".red()),
           Ordering::Greater => println!("{}","its something smaller than that!".red()),
        }
        println!("the correct number is: {}",randomnum);
    }

}

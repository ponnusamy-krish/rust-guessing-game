extern crate rand;
use std::io::stdin;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number!");
    println!("Plese input your guess");
    let secret_num= rand::thread_rng().gen_range(1..101);

    println!("The sectet Number is {}", secret_num);
    loop{


        let mut guess = String::new();
        
        stdin().read_line(&mut guess).expect("Failed to read line");
        let  guess: u32=match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("you guesssd: {}", guess);
        match guess.cmp(&secret_num){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {println!("You win");
            break;}
        }

    }    

}

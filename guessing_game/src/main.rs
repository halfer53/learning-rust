use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 100);

    let val = 98_222_1;

    println!("val {}", val);

    loop{
        println!("Plz provide a number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess : i32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue
        };

        println!("you guessed: {} ", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("less"),
            Ordering::Equal => {
                println!("you win");
                break;
            },
            Ordering::Greater => println!("greater")
        }
    }
}
    

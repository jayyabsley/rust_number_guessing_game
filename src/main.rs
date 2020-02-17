use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Lets play a number guessing game!");
    
    let rand_num = rand::thread_rng().gen_range(1, 101);
    //println!("[ADMIN] rnd: {}", rand_num);

    loop {
        println!("Please input your guess...");

        //Readline in from stdin(Console)
        let mut number_guessed = String::new();

        io::stdin().read_line(&mut number_guessed)
            .expect("Failed to process");

        //Convert val to unsigned 32bit int
        let number_guessed: u32 = match number_guessed.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Thats not a number you numpty");
                continue;
            },
        };

        println!("You guessed: {}", number_guessed);

        match number_guessed.cmp(&rand_num) {
            Ordering::Less => println!("Go higher!"),
            Ordering::Greater => println!("Lower!"),
            Ordering::Equal => {
                println!("Good for you Glenn Coco");
                break;
            }
        }
    }
    
}

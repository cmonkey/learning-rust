use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let tup = (500, 6.4 , false);

    let (x,y,z) = tup;

    println!("tup z = {}", z);

    println!("tuple = {}, {}, {}", tup.0, tup.1, tup.2);

    let array = [1,2,3,4,5,6];

    println!("array [0] = {}", array[0]);

    let array:[i32; 5] = [1,2,3,4,5];

    println!("array [4] = {}", array[4]);

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is : {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too smaill"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }

        }
    }
}

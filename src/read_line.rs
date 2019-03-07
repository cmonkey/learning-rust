use std::io;

fn main(){
    println!("What your name?");
    let mut name = String::new();

    match io::stdin().read_line(&mut name){
        OK(_) => println!("hello, {}", name),
        Error(e) => println!("Sorry, {}", e)
    }
}

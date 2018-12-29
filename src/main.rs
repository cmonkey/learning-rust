use std::io;
use rand::Rng;
use std::cmp::Ordering;

struct User{
    userName: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32,i32,i32);

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {

    let tup = (500, 6.4 , false);

    let (x,y,z) = tup;

    println!("tup z = {}", z);

    println!("tuple = {}, {}, {}", tup.0, tup.1, tup.2);

    let array = [1,2,3,4,5,6];

    println!("array [0] = {}", array[0]);

    let array:[i32; 5] = [1,2,3,4,5];

    println!("array [4] = {}", array[4]);

    for arr in array.iter() {
        println!("arr = {}", arr);
    }

    for number in (1..10) {
        println!("number = {}", number);
    }

    for number in (1..10).rev() {

        println!("rev number = {}", number);
    }

    let s = String::from("hello");

    take_string(s);

    // println!("s = {}", s); s 离开作用域，没有所有权
    
    let lenS = String::from("hello");
    let len = calculate_length(&lenS);

    let mut some_str = String::from("hello");

    change(&mut some_str);

    println!("some change = {}", some_str);

    println!("lenS = {}, len = {}", lenS, len);

    let plus_one = plus_one(10);

    println!("plus_one = {}", plus_one);

    let mut user1 = User{
		email:String::from("42.codemonkey@gmail.com"),
		userName:String::from("cmonkey"),
		active:true,
		sign_in_count: 1,
	};

	println!("user1 email = {}", user1.email);

	user1.email = String::from("42.codemonkey at gmail.com");

	println!("user1 mut email = {}", user1.email);

	let user2 = User{
        email:String::from("42.codemonkey@gmail.com"),
        userName:String::from("cmonkey"),
        ..user1
    };

    println!("user2.active = {} user1.active = {}", user2.active, user1.active);

    let back = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rect1 = Rectangle{width: 30, height:50};

    println!("rect1 is {:?}", rect1);

    println!(
      "The area of the rectangle is {} square pixels.", rect1.area()
      );

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

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn take_string(s: String) {
    println!("s = {}", s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String){
    some_string.push_str(" world");
}

fn build_user(email:String, userName:String) -> User {
 User{
   email,
   userName,
   active: true,
   sign_in_count: 1,
   }
}


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn get_user_input() -> String {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    return guess;
}

fn produce_secret_number()-> u32 {
    return rand::thread_rng().gen_range(1..=100);
}

fn parse_guess_as_number(guess: String)->u32{
    let guess_u32: u32 = guess.trim().parse().expect("Num?");
    return guess_u32
}

fn compare_guess_with_secret(guess: u32, secret: u32) {
     match guess.cmp(&secret){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
     }
}

fn main() {
    let secret = produce_secret_number();
    println!("[DEBUG:-)] Secret: {}", secret);

    let guess = get_user_input();
    println!("Guess: {}", guess);

    let guess_number:u32 = parse_guess_as_number(guess);

    compare_guess_with_secret( guess_number, secret);
}


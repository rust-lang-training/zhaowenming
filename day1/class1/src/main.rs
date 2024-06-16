use rand::Rng;
use std::{cmp::Ordering, io, vec};

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The serect number is: {secret_number}");

    loop {
        println!("please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("TOO small!"),
            Ordering::Equal => {
                println!("Your win!");
                break;
            }
            Ordering::Greater => println!("Too large!"),
        }
        println!("Your guessed: {guess}");
    }

    // assert_eq!(10_i8 as u16, 10_u16)
    // 10u8.checked_add(100);

    // 128u8.checked_add(200);

    // 128u8.checked_mul(3);
}
#[test]
fn test() {
    println!("{}", 123);
    let S1 = "Hello word";
    println!("S1 = {S1}");

    // let v: Vec<f64> = vec
}

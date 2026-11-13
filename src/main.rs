use std::io;

fn token_rng(found: &mut bool) {
    let num = rand::random_range(1..=140000);

    if num == 1 {
        println!("PIRATE TOKEN FOUND!");
        *found = true;
    }
 
}
fn main() {
    let mut input = String::new();
    let mut found_token = false;

    println!("How many times you want the program to roll for a token (1/140k chance)");
    io::stdin().read_line(&mut input).expect("Something went wrong.");

    let input: i32 = input.trim().parse().unwrap_or_else(|_| {
        println!("Please enter a valid integer.");
        std::process::exit(1);
    });


    for _ in 0..input {
        token_rng(&mut found_token);
    }

    if found_token {
        println!("At least one token was found. Congrats!");
    }
    else {
        println!("No tokens were found. Try again!");
    }
}

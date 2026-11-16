use std::io;

fn token_rng(rolls: u32) -> u32 {
    let mut count = 0;

    for _ in 0..rolls {
        let num = rand::random_range(1..=140_000);

        if num == 1 {
            println!("PIRATE TOKEN FOUND!");
            count += 1;
        }
    }

    count
}
fn main() {
    let mut input = String::new();

    println!("How many times you want the program to roll for a token (1/140k chance)");
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong.");

    let input: u32 = input.trim().parse().unwrap_or_else(|_| {
        println!("Please enter a valid integer.");
        std::process::exit(1);
    });

    let token_found = token_rng(input);

    if token_found != 0 {
        println!("Tokens found: {}, congratulations!", token_found);
    } else {
        println!("No tokens were found.");
    }
}

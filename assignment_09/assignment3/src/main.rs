use std::io;
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        return 0;
    } else if guess > secret {
        return 1;
    } else {
        return -1;
    }
}
fn main() {
    let mut count = 0;
    let mut my_secret = 504;
    loop {
        println!("Enter secret number: ");
        let mut input_secret = String::new();
        io::stdin()
            .read_line(&mut input_secret)
            .expect("Failed to read.");
        let guess: i32 = input_secret.trim().parse().expect("Put a valid integer.");
        let got = check_guess(guess, my_secret);
        count += 1;
        if got == 0 {
            println!("Correct Guess!");
            break;
        } else if got == 1 {
            println!("Your guess was too high! :(");
        } else if got == -1 {
            println!("Your guess was too low! :(");
        }
    }
    println!("Final guess count: {}", count);
}

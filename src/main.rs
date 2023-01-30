use std::io::{self, Write, BufReader, BufRead, Error};
use std::cmp::Ordering;
use rand::Rng;
use std::fs::File;

fn get_guess() -> u32 {
    let mut guess = String::new();
    io::stdin()
	.read_line(&mut guess)
	.expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
	Ok(num) => num,
	Err(_) => {
	    println!("Please enter an integer 0-100");
	    return get_guess();
	},
    };
    return guess;
}

fn eval_guess(playing: &mut bool, secret_number: &mut u32, guess: u32, strikes: &mut u32, score: &mut u32) {
    match guess.cmp(&secret_number) {
	Ordering::Less => {
	    println!("Too small!");
	    *strikes += 1;
	}
	Ordering::Greater =>{
	    println!("Too big!");
	    *strikes += 1;
	}
	Ordering::Equal => {
	    println!("You win!");
	    *score += 1;
	    if *score >= 3 {
		println!("Continue?(y/n)");
		let mut cont_response = String::new();
		io::stdin()
		    .read_line(&mut cont_response)
		    .expect("Failed to read line");
		if !cont_response.eq(&String::from("Y")) || !cont_response.eq(&String::from("y")) {
		    *playing = false;
		    return;
		}
	    }
	    *secret_number = rand::thread_rng().gen_range(1..=100);
	    println!("New random number is {secret_number}");
	}
    }
}

// fn read_data_file() -> Result<(), Error> {
//     let path = "game_data.csv";
//     let input = File::open(path)?;
//     let buffered = BufReader::new(input);

//     for line in buffered.lines() {
// 	println!("{}", line?);
//     }

//     Ok(())
// }

fn main() {
    // get the scoreboard from the file
    // read_data_file().expect("Failed to read from file");
    
    println!("guess the number!");

    let mut secret_number = rand::thread_rng().gen_range(1..=100);
    let mut score = 0;
    let mut strikes = 0;

    println!("the secret number is: {secret_number}");

    let mut playing: bool = true;

    while playing {
	println!("please input your guess.");

	let guess = get_guess();
	
	println!("You guessed: {guess}");

	eval_guess(&mut playing, &mut secret_number, guess, &mut strikes, &mut score);

	println!("Strikes= {}, Score= {}", strikes, score);
	
	if strikes >= 3 {
	    playing = false;
	}
    }
    // write to the score file
    println!("Goodbye!");
}

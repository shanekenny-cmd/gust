use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("guess the number!");

    let mut secret_number = rand::thread_rng().gen_range(1..=100);
    let mut score = 0;
    let mut strikes = 0;

    println!("the secret number is: {secret_number}");

    let mut playing: bool = true;

    while playing {
	println!("please input your guess.");

	let mut guess = String::new();

	io::stdin()
	    .read_line(&mut guess)
	    .expect("Failed to read line");

	let guess: u32 = match guess.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};
	
	println!("You guessed: {guess}");

	match guess.cmp(&secret_number) {
	    Ordering::Less => {
		println!("Too small!");
		strikes += 1;
	    }
	    Ordering::Greater =>{
		println!("Too big!");
		strikes += 1;
	    }
	    Ordering::Equal => {
		println!("You win!");
		score += 1;
		if score >= 3 {
		    println!("Continue?(y/n)");
		    let mut cont_response = String::new();
		    io::stdin()
			.read_line(&mut cont_response)
			.expect("Failed to read line");
		    if cont_response.eq(&String::from("Y")) || cont_response.eq(&String::from("y")) {
			playing = false;
		    }
		}
		secret_number = rand::thread_rng().gen_range(1..=100);
		println!("New random number is {secret_number}");
	    }
	}
	println!("Strikes= {}, Score= {}", strikes, score);
	if strikes >= 3 {
	    playing = false;
	}
	    
    }
    println!("Goodbye!");
}

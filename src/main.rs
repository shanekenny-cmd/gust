use std::io::{self, Write, BufReader, BufRead, Error};
use std::cmp::Ordering;
use std::collections::HashMap;
use rand::Rng;
use std::fs::OpenOptions;

fn get_guess() -> u32 {
    let mut guess = String::new();
    io::stdin()
    	.read_line(&mut guess)
       	.expect("failed to read line");
    let guess: u32 = match guess.trim().parse() {
    	Ok(num) => num,
    	Err(_) => {
    	    println!("please enter an integer 0-100");
    	    return get_guess();
    	},
    };
    return guess;
}

fn eval_guess(playing: &mut bool, secret_number: &mut u32, guess: u32, strikes: &mut u32, scoreboard: &mut HashMap<String, u32>, name: String) {
    match guess.cmp(&secret_number) {
    	Ordering::Less => {
    	    println!("too small!");
    	    *strikes += 1;
    	}
    	Ordering::Greater =>{
    	    println!("too big!");
    	    *strikes += 1;
    	}
    	Ordering::Equal => {
    	    println!("you win!");
    	    scoreboard.insert(name.clone(), scoreboard.get(&name).expect("player not found") + 1 as u32);
    	    let score = scoreboard.get(&name).expect("player not found");	    
    	    if *score >= 3 {
        		println!("continue?(y/n)");
        		let mut cont_response = String::new();
        		io::stdin()
        		    .read_line(&mut cont_response)
        		    .expect("Failed to read line");
        		if !cont_response.eq(&String::from("Y\n")) && !cont_response.eq(&String::from("y\n")) {
        		    *playing = false;
        		    return;
        		}
    	    }
    	    *secret_number = rand::thread_rng().gen_range(1..=100);
    	    println!("new random number is {secret_number}");
    	}
    }
}

fn read_data_file(scoreboard: &mut HashMap<String, u32>) -> Result<(), Error> {
    let path = "game_data.csv";
    let input_result = OpenOptions::new().read(true).create(false).open(path);
    let input = match input_result {
        Ok(input) => input,
        Err(err) => {
            return Err(err);
        }
    };
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
    	let mut name = String::new();
    	let mut score = String::new();
    	let mut second: bool = false;

    	let line_string = String::from(line.unwrap());

    	for b in line_string.as_bytes() {

    	    if *b == 44 as u8 {
        		second = true;
        		continue;
    	    }

    	    if second {
        		// append to score
        		score.push_str(&(*b as char).to_string());
    	    } else {
        		// append to name
        		name.push_str(&(*b as char).to_string());
    	    }
    	}
    	let num_score: u32 = score.trim().parse()
    	    .expect("Something went wrong");
    	scoreboard.insert(name, num_score);
    }
    Ok(())
}

fn print_score_board(scoreboard: HashMap<String, u32>) {
    println!("the scoreboard:");
    let mut hash_vec: Vec<(&String, &u32)> = scoreboard.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));
    let i = 0;
    for (name, score) in hash_vec {
    	println!("{name}: {score}");
        if i > 5 { break; }
    }
}

fn get_player_name(scoreboard: HashMap<String, u32>) -> String {
    println!("enter your name: ");
    let mut player_name = String::new();
    io::stdin()
    	.read_line(&mut player_name)
    	.expect("Something went wrong");
    
    player_name.remove(player_name.len() - 1);
    if player_name.len() > 7 {
    	println!("7 characters maximum, try again.");
    	return get_player_name(scoreboard);
    }
    match scoreboard.get(&player_name) {
        Some(_score) => {
            println!("that name is already taken, be more original.");
            return get_player_name(scoreboard.clone());
        }
        None => {
            return player_name;
        }
    }
}

fn write_data_file(scoreboard: HashMap<String, u32>) -> Result<(), Error> {
    let path = "game_data.csv";
    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();
    for (name, score) in scoreboard {
        write!(output, "{}, {}\n", name, score).unwrap();
    }
    Ok(())
}

fn main() {
    // get the scoreboard from the file
    let mut scoreboard = HashMap::new();
    
    let read_result = read_data_file(&mut scoreboard);

    match read_result {
        Ok(()) => {
            print_score_board(scoreboard.clone());
        },
        Err(_) => {},
    }

    let player_name = get_player_name(scoreboard.clone());
    scoreboard.insert(player_name.to_string(), 0);
    
    println!("\nguess the number!");

    let mut secret_number = rand::thread_rng().gen_range(1..=100);
    let mut strikes = 0;

    let mut playing: bool = true;

    while playing {

    	 println!("please input your guess.");

    	let guess = get_guess();
	
    	println!("you guessed: {guess}");

    	eval_guess(&mut playing, &mut secret_number, guess, &mut strikes, &mut scoreboard, player_name.to_string());

    	let score = scoreboard.get(&player_name).expect("player not found");
    	println!("strikes= {}, score= {}", strikes, score);	
        	if strikes >= 3 {
        	    playing = false;
                println!("the secret number was {}", secret_number);
        	}
    }
    // write to the score file
    write_data_file(scoreboard).expect("Failed to write to file.");
    println!("goodbye!");
}

//use anyhow::{Context, Result}; // Error handling
use clap::Parser; // Parse the CLI args
use std::io::stdin;

/// Generate alphanumeric password based on the length provided by the user
#[derive(Parser)]
struct Cli {
    /// Length of the password
    pattern: String,
}

fn main() {
    println!("Enter desired length of the password...");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Did not enter a correct string");
    
    let psw_len = input.trim().parse::<u32>().unwrap_or(0);
    
    if psw_len <= 0{
        println!("Could not parse the length {}, try inputting a valid number", input.trim());
        return;
    }

    let max_len = 100;
    if psw_len > max_len {
        println!("Maximum length is {} characters, {} is too long", max_len, psw_len);
        return;
    }

    if psw_len > 0 {
        println!("Entered a number: {}", psw_len);
        
        let symbols = buttress::AllowedSymbols {
            lower:      true,
            upper:      true,
            numbers:    true,
            special:    true,
        };

        let password = buttress::generate_password(psw_len, symbols);
        println!("Here's your new password: {}", password);
    } 
}




/* 
fn main() {
    let args = Cli::parse();
    
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    
    //println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
}
*/

/* 
fn old_main() {
    use std::io::{stdin};
    use rand::{distributions::Alphanumeric, Rng}; // 0.8

    println!("Enter desired length of the password...");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Did not enter a correct string");
    
    let psw_len = input.trim().parse::<u32>().unwrap_or(0);
    println!("Entered a number: {}", psw_len);

    if psw_len > 0 {
        let str_len = psw_len.try_into().unwrap(); // convert u32 to usize
        let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(str_len)
        .map(char::from)
        .collect();

        println!("Here's your new password: {}", password);
    }
}
*/

//fn request_input() -> i32 {
    //use std::io::{stdin};
    //let mut input = String::new();
    //stdin().read_line(&mut input).expect("Did not enter a correct string");
    ////let length: u32 = input.parse().unwrap();
//
    //match input.parse::<u32>() {
    //    Ok(n) => return input.parse::<u32>(),
    //    Err(e) => return -1,
    //}
//}
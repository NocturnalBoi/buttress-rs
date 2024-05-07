//use std::vec;
#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;


pub struct AllowedSymbols {
    pub lower: bool,
    pub upper: bool,
    pub numbers: bool,
    pub special: bool,
}

// Generate an alphanumeric password of the length
pub fn generate_password(length: u32, allowed_symbols: &AllowedSymbols) -> String {
    use rand::Rng; // Random Number Generator

    let alpha_lower: &str       = "abcdefghijklmnopqrstuvwxyz";
    let alpha_upper: &str       = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers: &str           = "0123456789";
    let special_chars: &str     = "!#$%&?*+-='.,/:;^_`|<>@()[]{}~";

    let mut chars: String = String::new();
    if allowed_symbols.lower == true {
        chars = format!("{}{}", chars, alpha_lower);
    }
    
    if allowed_symbols.upper == true {
        chars = format!("{}{}", chars, alpha_upper);
    }

    if allowed_symbols.numbers == true {
        chars = format!("{}{}", chars, numbers);
    }
    
    if allowed_symbols.special == true {
        chars = format!("{}{}", chars, special_chars);
    }

    let mut password: String = String::new();
    if chars.len() > 0 {
        let mut rng = rand::thread_rng();
        let char_idx_max = chars.len() - 1;
        for _i in 0..length {
            let idx: usize = rng.gen_range(0..char_idx_max);    // get random int 
            let new_byte: u8 = chars.as_bytes()[idx];           // get bytees from the charset at the index
            let new_char: char = new_byte as char;              // convert bytes to char
            password.push(new_char);                        // append it to the password
        }
    }

    return password;
}

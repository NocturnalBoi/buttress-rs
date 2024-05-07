#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 400.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Buttress",
        native_options,
        Box::new(|cc| Box::new(buttress::TemplateApp::new(cc))),
    )
}

/* 
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
*/
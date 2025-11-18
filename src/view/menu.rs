use std::io::{self, Write};
use crate::controller; // imports controller

pub fn main_menu() -> Result<(), Box<dyn std::error::Error>> // returns ok(()) on success and store error in Box on failure (stores error on heap so size does not matter)
{
    loop
    {
        println!("\nSelect Language Implementation:");
        println!("[1] Load the file");
        println!("[2] Generate Reports");
        print!("\nEnter choice: ");
        io::stdout().flush()?; // makes sure prompt prints immediately

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?; // reads user input and store in choice
        let choice = choice.trim(); // removes whitespace

        match choice
        {
            "1" =>
            {
                controller::load_file()?; // calls controller to load file
            }
            "2" =>
            {
                controller::generate_reports()?; // calls controller to generate reports
            }
            _ => // user input is neither 1 or 2
            {
                println!("\nInvalid choice. Please enter 1 or 2.");
            }
        }
    }
}

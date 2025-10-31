use std::io::{self, Write};
use crate::controller;

pub fn main_menu() -> Result<(), Box<dyn std::error::Error>>
{
    loop
    {
        println!("\nSelect Language Implementation:");
        println!("[1] Load the file");
        println!("[2] Generate Reports");
        print!("\nEnter choice: ");
        io::stdout().flush()?; // make sure prompt prints immediately

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice
        {
            "1" =>
            {
                controller::load_file()?; // calls your controller logic
            }
            "2" =>
            {
                controller::generate_reports()?; // runs reports
            }
            _ =>
            {
                println!("\nInvalid choice. Please enter 1 or 2.");
            }
        }
    }
}

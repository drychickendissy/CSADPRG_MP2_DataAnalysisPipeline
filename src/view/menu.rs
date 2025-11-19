/********************
Last names: Abdulrahman, Bilanes, Cruz, Nicolas
Language: JavaScript
Paradigm(s): Procedural, Object-Oriented, Functional, Data-Driven, Immutable
********************/

use std::io::{self, Write};
use crate::controller; // imports controller

pub fn main_menu() -> Result<(), Box<dyn std::error::Error>> {
    let mut loaded_projects: Option<Vec<crate::model::Project>> = None;
    loop {
        println!("\nSelect Language Implementation:");
        println!("[1] Load the file");
        println!("[2] Generate Reports");
        print!("\nEnter choice: ");
        io::stdout().flush()?;  // ensure prompt prints immediately

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => {
                loaded_projects = Some(controller::load_file()?);
            }
            "2" => {
                if let Some(ref projects) = loaded_projects {
                    controller::generate_reports(projects)?;    // generate reports from controller
                } else {
                    println!("\nNo data loaded yet. Please choose [1] first.");
                }
            }
            _ => {
                println!("\nInvalid choice. Please enter 1 or 2.");
            }
        }
    }
}

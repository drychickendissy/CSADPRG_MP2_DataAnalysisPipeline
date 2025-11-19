/********************
Last names: Abdulrahman, Bilanes, Cruz, Nicolas
Language: JavaScript
Paradigm(s): Procedural, Object-Oriented, Functional, Data-Driven, Immutable
********************/

mod model;
mod controller;
mod view;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    view::menu::main_menu()?;
    Ok(())
}

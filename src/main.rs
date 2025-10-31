mod model;
mod controller;
mod view;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    view::menu::main_menu()?;
    Ok(())
}

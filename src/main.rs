mod model;
mod view;
mod controller;

fn main()
{
    if let Err(e) = controller::run_pipeline()
    {
        eprintln!("Error: {}", e);
    }
}

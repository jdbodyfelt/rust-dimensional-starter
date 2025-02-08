
mod locations;
use locations::LocationTable;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let name = String::from("cities");
    let mut table = LocationTable::new(name);
    table.run();
    table.display();

    Ok(())
}

mod locations;
use locations::LocationTable;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    for name in ["nations", "regions", "cities"].iter() {
        let mut table = LocationTable::new(name.to_string());
        table.run();
        table.display();
    }

    Ok(())
}
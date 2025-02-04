use polars::prelude::*;
use reqwest::blocking::get;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Fetch the data
    let url = "https://download.geonames.org/export/dump/countryInfo.txt";
    let response = get(url)?;
    let data = response.text()?;

    // Read the content into a DataFrame
    let cursor = Cursor::new(data);
    

    println!("{:?}", df);
    Ok(())
}
use reqwest::blocking::get;
use std::io::Cursor;

use polars::prelude::*;

fn main() -> Result<()> {
    // URL of the data
    let url = "https://download.geonames.org/export/dump/countryInfo.txt";

    // Fetch the data
    let response = get(url).expect("Failed to download file");
    let content = response.text().expect("Failed to read response text");

    // Filter out comment lines and create a CSV-like string
    let csv_data: String = content
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect::<Vec<&str>>()
        .join("\n");

    // Read the CSV data into a Polars DataFrame
    let mut reader = Cursor::new(csv_data);
    let df = CsvReader::new(&mut reader)
        .has_header(true)
        .finish()?;

    // Print the DataFrame
    println!("{:?}", df);

    Ok(())
}
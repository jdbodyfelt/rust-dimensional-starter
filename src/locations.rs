use std::fs::File;
use std::io::{Read, Write}; 
use reqwest::blocking::get; 

use polars_core::prelude::*;
use polars_io::prelude::*;

/*************************************************************************************/
pub struct LocationTable {
    name: String,
    url: String, 
    target: String,
    df: DataFrame
}

impl LocationTable {

    /* Class Constructor */
    pub fn new(name: String) -> Self {
        // Check for valid table name 
        let valids = vec![
            "nations", 
            "regions", 
            "cities"
        ];
        if !valids.contains(&name.as_str()) {
            panic!("Unknown table name: {}", name);
        }
        // Instantiate
        let mut url = String::from("https://download.geonames.org/export/dump/");
        let table = match name.as_str() {
            "nations" => "countryInfo.txt",
            "regions" => "admin1CodesASCII.txt",
            "cities" => "cities500.zip",
            _ => panic!("Invalid table name")
        };
        url.push_str(table);
        let target = name.clone() + ".tsv";
        let df = DataFrame::default();
        LocationTable {name,url,target,df}
    }

    /* Unzipping into a string (for city data file) */
    fn unzip(&mut self) -> Result<String, Box<dyn std::error::Error>>  {
        // Download
        let resp = get(self.url.clone())?.bytes()?;
        let reader = std::io::Cursor::new(resp);
        let mut archive = zip::ZipArchive::new(reader)?;
        let mut file = archive.by_index(0)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        return Ok(data);
    }

    /* Extract data from URL into local TSV */
    pub fn extract(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Pull the raw data
        let data = if self.name == "cities" {
            self.unzip()?
        } else {
            get(self.url.clone())?.text()?
        };
        println!("Data: {}", data);
        // Get the last line starting with '#' - this is the header!
        let mut header = data
            .lines()
            .rev()
            .find(|line| line.starts_with('#'))
            .unwrap_or("")
            .trim_start_matches('#')
            .trim()
            .to_lowercase();
        // Filter out all lines starting with '#'
        let data = data
            .lines()
            .filter(|line| !line.starts_with('#'))
            .collect::<Vec<&str>>()
            .join("\n");
        // Write the data to a TSV file
        let mut file = File::create(self.target.to_string())?;
        let region_cols = ["code", "name", "acii_name", "geoname_id"].join("\t");
        let city_cols = [
            "geoname_id", "name", "ascii_name", "alternate_names",
            "latitude", "longitude",
            "feature_class", "feature_code",
            "country_code", "cc2",
            "admin1_code", "admin2_code", "admin3_code", "admin4_code",
            "population", "elevation", "dem", "timezone",
            "mod_date"
        ].join("\t");
        header = match self.name.as_str() {
            "nations" => header,
            "regions" => region_cols,
            "cities" => city_cols,
            _ => panic!("Invalid table name")
        };
        file.write_all(header.as_bytes())?;
        file.write_all(String::from("\n").as_bytes())?;
        file.write_all(data.as_bytes())?;
        println!("WROTE: {}", self.target);
        // Finally, load the data into a dataframe
        let target = self.target.clone();
        println!("Target: {}", target);
        self.df = CsvReadOptions::default()
            .with_has_header(true)
            .with_parse_options(
                CsvParseOptions::default()
                    .with_separator(b'\t')
                    .with_encoding(CsvEncoding::Utf8)
                    .with_try_parse_dates(true)
            )
            .try_into_reader_with_file_path(Some(target.into()))?
            .finish()?;
        Ok(())
    }

    /* Transform the TSV data into a local dataframe */
    pub fn transform(&mut self) -> Result<(), Box<dyn std::error::Error>> {
 


        Ok(())
    }

    pub fn clean(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::remove_file(&self.target)?;
        Ok(())
    }

    /* Load the local dataframe into a database target */
    pub fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    /* Run the full data ETL */
    pub fn run(&mut self) {
        let _ = self.extract();
        let _ = self.transform(); 
        let _ = self.load();
        //let _ = self.clean(); 
    }

    // Print Check
    pub fn display(&self) {
        println!("Table: {}", self.name);
        println!("Src: {}", self.url);
        println!("Data: {:?}", self.df.head(Some(20)));
    }
}
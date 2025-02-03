# rust-dimensional-starter

Many data designs require a set of dimensional tables to address standardised information - for example: locations, occupations, currencies, or colors. This project pulls such data from online sources. The data is prepared as either an **enumerator** or a **reference** - the former has a strict row schema of {`category_id` (unique PK), `category_name`, `category_description`}, while the latter is a bit freer with {`reference_id` (unique PK), `reference_name`, `...<other columns>`}. Prepared data is then injected into either an `enum__` or `ref__` prepended table within a database, or dumped to CSV. 

In addition to providing a nifty "jump-starter" for many different data projects, this GitHub project also serves as a sandbox for the original author [@jdbodyfelt](https://www.github.com/jdbodyfelt) to hone his Rust chops!

## Usage

## Data Sources

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize, Serialize)]
struct LottoNumber {
    number: String,
    date: String,
    prize: String,
    reward: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read JSON file
    let json_content = std::fs::read_to_string("lotto_numbers.json")?;
    let numbers: Vec<LottoNumber> = serde_json::from_str(&json_content)?;

    // Create CSV file
    let mut csv_file = File::create("lotto_numbers.csv")?;
    
    // Write header
    writeln!(csv_file, "number,date,prize,reward")?;

    // Write data
    for number in numbers {
        writeln!(
            csv_file,
            "{},{},{},{}",
            number.number,
            number.date,
            number.prize,
            number.reward
        )?;
    }

    println!("Successfully converted to CSV!");
    Ok(())
} 
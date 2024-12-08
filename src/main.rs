use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use csv::{Reader, Writer};

fn deduplicate_csv(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    // Open the input CSV file
    let input_file = File::open(input_path)?;
    let reader = Reader::from_reader(BufReader::new(input_file));

    // Create the output CSV file
    let output_file = File::create(output_path)?;
    let mut writer = Writer::from_writer(BufWriter::new(output_file));

    // Use a HashSet to track unique lines
    let mut seen_lines = HashSet::new();

    // Read and process each record
    for result in reader.into_records() {
        let record = result?;

        // Convert the record to a string for deduplication
        let line_key = record
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(",");

        // Only write the record if it hasn't been seen before
        if seen_lines.insert(line_key) {
            writer.write_record(&record)?;
        }
    }

    // Flush and close the writer
    writer.flush()?;

    println!(
        "Deduplication complete. Unique lines written to {}",
        output_path
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if correct number of arguments is provided
    if args.len() != 3 {
        eprintln!("Usage: {} <input_csv> <output_csv>", args[0]);
        std::process::exit(1);
    }

    // Extract input and output file paths
    let input_path = &args[1];
    let output_path = &args[2];

    // Verify input file exists
    if !Path::new(input_path).exists() {
        eprintln!("Error: Input file '{}' does not exist", input_path);
        std::process::exit(1);
    }

    // Perform CSV deduplication
    deduplicate_csv(input_path, output_path)
}


use std::fs;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use pysmennyi_phone_parser::*;

#[derive(Parser)]
#[command(
    name = "phone_number_parser",
    version = "1.0.0",
    about = "Phone number parser",
    author = "Anton Pysmennyi"
)]
struct Cli {
    input: String,
    output: String,
    #[command(subcommand)]
    mode: Option<Modes>,
}

#[derive(Subcommand)]
enum Modes {
    ConvertToE164,
    ExtractCountryCodes,
    ExtractOperatorCodes,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.mode {
        Some(Modes::ConvertToE164) => {
            println!("Converting '{}' to E.164 format...", cli.input);
            // handle_convert_to_e164(&cli.input);
        }
        Some(Modes::ExtractCountryCodes) => {
            println!("Extracting country codes from '{}'...", cli.input);
            handle_extract_country_codes(&cli.input, &cli.output)?;
        }
        Some(Modes::ExtractOperatorCodes) => {
            println!("Extracting operator codes from '{}'...", cli.input);
            handle_extract_operator_codes(&cli.input, &cli.output)?;
        }
        None => {
            println!("No mode selected. Provide a subcommand to proceed.");
        }
    }
    Ok(())
    // let input = "00 380 (50) 123 45 67";
    // let phones = Phone::from_string(input).unwrap();
    // println!("{:?}", phones);
}

fn handle_extract_operator_codes(input: &str, output: &str) -> Result<()> {
    let file_content =
        fs::read_to_string(input).with_context(|| format!("Could not read file: {}", input))?;
    let operator_codes = extract_operator_codes(file_content)?;
    let result = operator_codes.join("\n");

    fs::write(output, result).with_context(|| "Could not write to output file")?;

    println!("Codes extracted to '{}'", output);
    Ok(())
}

fn handle_extract_country_codes(input: &str, output: &str) -> Result<()> {
    let file_content =
        fs::read_to_string(input).with_context(|| format!("Could not read file: {}", input))?;
    let country_codes = extract_country_codes(file_content)?;
    let result = country_codes.join("\n");
    println!("{}", result);
    Ok(())
}

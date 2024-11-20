use std::fs;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use pysmennyi_phone_parser::*;

#[derive(Parser)]
#[command(
    name = "phone_number_parser",
    version = "1.0.0",
    about = "Phone number parser",
    author = "Anton Pysmennyi",
    subcommand_value_name = "MODE"
)]
struct Cli {
    #[arg(help = "Provide the file path with the phones that need to be processed")]
    input_file: String,
    #[arg(help = "Provide the file path where the result will be saved")]
    output_file: String,
    #[command(subcommand)]
    mode: Option<Modes>,
}

#[derive(Subcommand)]
enum Modes {
    #[command(
        about = "In this mode the program converts phone numbers to the standardized E.164 format"
    )]
    ConvertToE164,
    #[command(about = "In this mode the program extracts country codes from the phone numbers")]
    ExtractCountryCodes,
    #[command(about = "In this mode the program extracts operator codes from the phone numbers")]
    ExtractOperatorCodes,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.mode {
        Some(Modes::ConvertToE164) => {
            println!("Converting '{}' to E.164 format...", cli.input_file);
            handle_convert_to_e164(&cli.input_file, &cli.output_file)?;
        }
        Some(Modes::ExtractCountryCodes) => {
            println!("Extracting country codes from '{}'...", cli.input_file);
            handle_extract_country_codes(&cli.input_file, &cli.output_file)?;
        }
        Some(Modes::ExtractOperatorCodes) => {
            println!("Extracting operator codes from '{}'...", cli.input_file);
            handle_extract_operator_codes(&cli.input_file, &cli.output_file)?;
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
    fs::write(output, result).with_context(|| "Could not write to output file")?;

    println!("Codes extracted to '{}'", output);
    Ok(())
}

fn handle_convert_to_e164(input: &str, output: &str) -> Result<()> {
    let file_content =
        fs::read_to_string(input).with_context(|| format!("Could not read file: {}", input))?;
    let e164_numbers = convert_to_e164(file_content)?;
    let result = e164_numbers.join("\n");
    fs::write(output, result).with_context(|| "Could not write to output file")?;

    println!(
        "Numbers converted to E.164 format and saved to '{}'",
        output
    );
    Ok(())
}

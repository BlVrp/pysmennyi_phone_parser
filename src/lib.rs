use anyhow::{Context, Result};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct PhoneParser;

#[derive(Debug)]
pub struct Phone {
    pub country_code: String,
    pub operator_code: String,
    pub local_phone_number: String,
}

impl Phone {
    pub fn from_pair(pair: pest::iterators::Pair<Rule>) -> Result<Self, anyhow::Error> {
        let mut inner_rules = pair
            .into_inner()
            .next()
            .context("Failed to parse top-level rule")?
            .into_inner();

        let _plus = inner_rules.next().context("Missing '+' symbol")?;

        let country_code = inner_rules
            .next()
            .context("Missing country code")?
            .as_str()
            .to_string();

        inner_rules
            .next()
            .context("Missing whitespace after country code")?;

        let operator_code = inner_rules
            .next()
            .context("Missing operator code")?
            .as_str()
            .to_string();

        let local_phone_number = inner_rules
            .filter(|p| {
                p.as_rule() == Rule::local_number_group || p.as_rule() == Rule::local_number_block
            })
            .map(|p| p.as_str().trim())
            .collect::<Vec<_>>()
            .join("");

        if local_phone_number.is_empty() {
            anyhow::bail!("Missing local phone number");
        }

        Ok(Phone {
            country_code,
            operator_code,
            local_phone_number,
        })
    }

    pub fn from_string(input: &str) -> Result<Vec<Self>> {
        let mut parsed_numbers = Vec::new();

        for (line_number, row) in input.lines().enumerate() {
            if row.trim().is_empty() {
                continue;
            }

            let mut parsed =
                PhoneParser::parse(Rule::phone_number, row.trim()).with_context(|| {
                    format!("Failed to parse phone number on line {}", line_number + 1)
                })?;

            let pair = parsed.next().context("No phone number to be parsed")?;

            let phone = Phone::from_pair(pair).with_context(|| {
                format!("Failed to parse phone number on line {}", line_number + 1)
            })?;
            parsed_numbers.push(phone);
        }
        Ok(parsed_numbers)
    }
}

pub fn extract_operator_codes(input: String) -> Result<Vec<String>> {
    let phones = Phone::from_string(&input)?;
    let operator_codes = phones
        .iter()
        .map(|phone| phone.operator_code.clone())
        .collect::<Vec<_>>();
    Ok(operator_codes)
}

pub fn extract_country_codes(input: String) -> Result<Vec<String>> {
    let phones = Phone::from_string(&input)?;
    let country_codes = phones
        .iter()
        .map(|phone| phone.country_code.clone())
        .collect::<Vec<_>>();
    Ok(country_codes)
}

pub fn convert_to_e164(input: String) -> Result<Vec<String>> {
    let phones = Phone::from_string(&input)?;
    let e164_numbers = phones
        .iter()
        .map(|phone| {
            format!(
                "+{}{}{}",
                phone.country_code, phone.operator_code, phone.local_phone_number
            )
        })
        .collect::<Vec<_>>();
    Ok(e164_numbers)
}

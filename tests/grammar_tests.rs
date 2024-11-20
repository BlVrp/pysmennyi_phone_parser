use anyhow::{Context, Result};
use pest::Parser;

use pysmennyi_phone_parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_international_grouped_phone_number() -> Result<()> {
        let input = "+380 50 123 45 67";

        let parsed = PhoneParser::parse(Rule::phone_number, input)
            .context("Failed to parse the phone number")?
            .next()
            .context("No match found in the parsed input")?;

        let phone = Phone::from_pair(parsed).context("Failed to convert parsed pair to Phone")?;

        assert_eq!(phone.country_code, "380");
        assert_eq!(phone.operator_code, "50");
        assert_eq!(phone.local_phone_number, "1234567");

        Ok(())
    }

    #[test]
    fn parses_international_block_phone_number() -> Result<()> {
        let input = "+380 50 1234567";

        let parsed = PhoneParser::parse(Rule::phone_number, input)
            .context("Failed to parse the phone number")?
            .next()
            .context("No match found in the parsed input")?;

        let phone = Phone::from_pair(parsed).context("Failed to convert parsed pair to Phone")?;

        assert_eq!(phone.country_code, "380");
        assert_eq!(phone.operator_code, "50");
        assert_eq!(phone.local_phone_number, "1234567");

        Ok(())
    }

    #[test]
    fn parses_international_block_phone_number_with_braces() -> Result<()> {
        let input = "+380 (50) 1234567";

        let parsed = PhoneParser::parse(Rule::phone_number, input)
            .context("Failed to parse the phone number")?
            .next()
            .context("No match found in the parsed input")?;

        let phone = Phone::from_pair(parsed).context("Failed to convert parsed pair to Phone")?;

        assert_eq!(phone.country_code, "380");
        assert_eq!(phone.operator_code, "50");
        assert_eq!(phone.local_phone_number, "1234567");

        Ok(())
    }

    #[test]
    fn parses_only_start_of_input_if_its_too_long() -> Result<()> {
        let input = "+380 123 4567 8888 8888 99";

        let parsed = PhoneParser::parse(Rule::phone_number, input)
            .context("Failed to parse the phone number")?
            .next()
            .context("No match found in the parsed input")?;

        let phone = Phone::from_pair(parsed).context("Failed to convert parsed pair to Phone")?;

        assert_eq!(phone.country_code, "380");
        assert_eq!(phone.operator_code, "123");
        assert_eq!(phone.local_phone_number, "456788888888");

        Ok(())
    }

    #[test]
    fn parses_double_zero_grouped_phone_number_with_braces() -> Result<()> {
        let input = "00 380 (50) 123 45 67";

        let parsed = PhoneParser::parse(Rule::phone_number, input)
            .context("Failed to parse the phone number")?
            .next()
            .context("No match found in the parsed input")?;

        let phone = Phone::from_pair(parsed).context("Failed to convert parsed pair to Phone")?;

        assert_eq!(phone.country_code, "380");
        assert_eq!(phone.operator_code, "50");
        assert_eq!(phone.local_phone_number, "1234567");

        Ok(())
    }

    #[test]
    fn fails_to_parse_phone_number_with_missing_country_code() {
        let input = "50 123 45 67";

        let parsed = PhoneParser::parse(Rule::phone_number, input);

        assert!(parsed.is_err());
    }

    #[test]
    fn fails_to_parse_phone_number_with_local_number_too_small() {
        let input = "+380 123 7";

        let parsed = PhoneParser::parse(Rule::phone_number, input);

        assert!(parsed.is_err());
    }
}

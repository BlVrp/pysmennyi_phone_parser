use anyhow::anyhow;
use pest::Parser;

use pysmennyi_phone_parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phone_parser() {
        let input = "+380 50 123 45 67";
        let parsed = PhoneParser::parse(Rule::phone_number, input)
            .map_err(|e| anyhow!("{}", e))
            .unwrap()
            .next()
            .unwrap();
        let phone = Phone::from_pair(parsed).unwrap();
        assert_eq!(phone.country_code, "380");
        assert_eq!(phone.operator_code, "50");
        assert_eq!(phone.local_phone_number, "1234567");
    }
}

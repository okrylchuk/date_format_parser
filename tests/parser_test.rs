use anyhow::Result;
use date_format_parser::parse_date; // Replace `date_format_parser` with your crate name

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for date_time rule
    #[test]
    fn test_date_and_time_full() -> Result<()> {
        let input = "12-20-2000 14:30:45.123";
        let result = parse_date(input)?;
        assert_eq!(result, "2000-12-20T14:30:45.123");
        Ok(())
    }

    #[test]
    fn test_date_and_time_full_slash() -> Result<()> {
        let input = "2000/12/20 14:30:45.123";
        let result = parse_date(input)?;
        assert_eq!(result, "2000-12-20T14:30:45.123");
        Ok(())
    }

    #[test]
    fn test_date_and_time_full_dots() -> Result<()> {
        let input = "2000.12.20 14:30:45.123";
        let result = parse_date(input)?;
        assert_eq!(result, "2000-12-20T14:30:45.123");
        Ok(())
    }

    #[test]
    fn test_date_and_time_without_milliseconds() -> Result<()> {
        let input = "2000-12-20 14:30:45";
        let result = parse_date(input)?;
        assert_eq!(result, "2000-12-20T14:30:45");
        Ok(())
    }

    #[test]
    fn test_date_and_time_without_seconds() -> Result<()> {
        let input = "2000-12-20 14:30";
        let result = parse_date(input)?;
        assert_eq!(result, "2000-12-20T14:30");
        Ok(())
    }

    // Tests for date rule
    #[test]
    fn test_date_with_dashes() -> Result<()> {
        let input = "2000-12-20";
        let result = parse_date(input)?;
        assert_eq!(result, "2000-12-20");
        Ok(())
    }

    #[test]
    fn test_date_with_slashes() -> Result<()> {
        let input = "12/20/2000";
        let result = parse_date(input)?;
        assert_eq!(result, "2000-12-20");
        Ok(())
    }

    #[test]
    fn test_date_with_dots() -> Result<()> {
        let input = "20.12.2000";
        let result = parse_date(input)?;
        assert_eq!(result, "2000-12-20");
        Ok(())
    }

    // Tests for year_month_day rule
    #[test]
    fn test_year_month_day_format() -> Result<()> {
        let input = "2000/12/20";
        let result = parse_date(input)?;
        assert_eq!(result, "2000-12-20");
        Ok(())
    }

    // Tests for day_month_year rule
    #[test]
    fn test_day_month_year_format() -> Result<()> {
        let input = "20.12.2000";
        let result = parse_date(input)?;
        assert_eq!(result, "2000-12-20");
        Ok(())
    }

    // Tests for month_day_year rule
    #[test]
    fn test_month_day_year_format() -> Result<()> {
        let input = "12/20/2000";
        let result = parse_date(input)?;
        assert_eq!(result, "2000-12-20");
        Ok(())
    }

    // Tests for year rule
    #[test]
    fn test_year() -> Result<()> {
        let input = "2024";
        let result = parse_date(&format!("{}-12-20", input))?;
        assert_eq!(result, "2024-12-20");
        Ok(())
    }

    #[test]
    fn test_year_too_long() {
        let input = "20245";
        let result = parse_date(&format!("{}-12-20", input));
        assert!(
            result.is_err(),
            "Expected an error for year with too many digits, but parsing succeeded."
        );
    }

    #[test]
    fn test_year_too_short() {
        let input = "20";
        let result = parse_date(&format!("{}-12-20", input));
        assert!(
            result.is_err(),
            "Expected an error for year with too few digits, but parsing succeeded."
        );
    }

    // Tests for month rule
    #[test]
    fn test_month() -> Result<()> {
        let input = "12";
        let result = parse_date(&format!("2000-{}-20", input))?;
        assert_eq!(result, "2000-12-20");
        Ok(())
    }

    #[test]
    fn test_month_too_short() {
        let input = "2";
        let result = parse_date(&format!("2020-{}-20", input));
        assert!(
            result.is_err(),
            "Expected an error for month with too few digits, but parsing succeeded."
        );
    }

    // Tests for day rule
    #[test]
    fn test_day() -> Result<()> {
        let input = "31";
        let result = parse_date(&format!("2000-12-{}", input))?;
        assert_eq!(result, "2000-12-31");
        Ok(())
    }

    #[test]
    fn test_invalid_day() {
        let input = "33";
        let result = parse_date(&format!("2020-12-{}", input));
        assert!(
            result.is_err(),
            "Expected an error for invalid day, but parsing succeeded."
        );
    }

    // Test for time rule
    #[test]
    fn test_time_full() -> Result<()> {
        let input = "14:30:45.123";
        let result = parse_date(&format!("2000-12-20 {}", input))?;
        assert_eq!(result, "2000-12-20T14:30:45.123");
        Ok(())
    }

    // Test for hour_24 rule
    #[test]
    fn test_hour_24() -> Result<()> {
        let input = "23";
        let result = parse_date(&format!("2000-12-20 {}:59:59", input))?;
        assert_eq!(result, "2000-12-20T23:59:59");
        Ok(())
    }

    // Test for minute rule
    #[test]
    fn test_minute() -> Result<()> {
        let input = "59";
        let result = parse_date(&format!("2000-12-20 14:{}", input))?;
        assert_eq!(result, "2000-12-20T14:59");
        Ok(())
    }

    // Test for second rule
    #[test]
    fn test_second() -> Result<()> {
        let input = "59";
        let result = parse_date(&format!("2000-12-20 14:30:{}", input))?;
        assert_eq!(result, "2000-12-20T14:30:59");
        Ok(())
    }

    // Test for millisecond rule
    #[test]
    fn test_millisecond() -> Result<()> {
        let input = "999";
        let result = parse_date(&format!("2000-12-20 14:30:45.{}", input))?;
        assert_eq!(result, "2000-12-20T14:30:45.999");
        Ok(())
    }

    // Tests for separator rule
    #[test]
    fn test_separator_dash() -> Result<()> {
        let input = "2000-12-20";
        let result = parse_date(input)?;
        assert_eq!(result, "2000-12-20");
        Ok(())
    }

    #[test]
    fn test_separator_slash() -> Result<()> {
        let input = "2000/12/20";
        let result = parse_date(input)?;
        assert_eq!(result, "2000-12-20");
        Ok(())
    }

    #[test]
    fn test_separator_dot() -> Result<()> {
        let input = "20.12.2000";
        let result = parse_date(input)?;
        assert_eq!(result, "2000-12-20");
        Ok(())
    }

    // Test for space
    #[test]
    fn test_space() -> Result<()> {
        let input = " ";
        let result = parse_date(&format!("2000-12-20{}14:30:45.123", input))?;
        assert_eq!(result, "2000-12-20T14:30:45.123");
        Ok(())
    }
}

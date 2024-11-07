use pest::Parser;
use date_format_parser::*;
use anyhow::{Result, anyhow, Context};



#[test]
fn dmy_format_recognition() -> Result<()> {
    let pair = Grammar::parse(Rule::date, "20/12/2000")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "20/12/2000");
    Ok(())
}


#[test]
fn test_dmy_format() -> Result<()> {
    let input_date = "20/12/2000";
    let parsed_date = Grammar::parse(Rule::date, input_date)
        .context("Failed to parse date")?;

    let mut day = "";
    let mut month = "";
    let mut year = "";

    for record in parsed_date {
        for pair in record.into_inner() {
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::day => day = inner_pair.as_str(),
                    Rule::month => month = inner_pair.as_str(),
                    Rule::year => year = inner_pair.as_str(),
                    _ => {}
                }
            }
        }
    }

    assert_eq!(year, "2000");
    assert_eq!(month, "12");
    assert_eq!(day, "20");

    Ok(())
}

#[test]
fn test_mdy_format() -> Result<()> {
    let input_date = "12-20-2000";
    let parsed_data = Grammar::parse(Rule::date, input_date)
        .context("Failed to parse date in MDY format")?;

    let mut day = "";
    let mut month = "";
    let mut year = "";

    for record in parsed_data {
        for pair in record.into_inner() {
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::day => day = inner_pair.as_str(),
                    Rule::month => month = inner_pair.as_str(),
                    Rule::year => year = inner_pair.as_str(),
                    _ => {}
                }
            }
        }
    }

    assert_eq!(year, "2000");
    assert_eq!(month, "12");
    assert_eq!(day, "20");

    Ok(())
}


#[test]
fn test_invalid_date_format() -> Result<()> {
    let input_date = "2000,12,20";
    let result = Grammar::parse(Rule::date, input_date);
    assert!(result.is_err(), "Invalid date format");

    Ok(())
}

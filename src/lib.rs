use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Failed to parse input: invalid date or time format")]
    InvalidInput,

    #[error("No match found in the input")]
    NoMatchFound,
}

pub fn parse_date(input: &str) -> Result<String, ParseError> {
    let mut pairs = Grammar::parse(Rule::date_time, input).map_err(|_| ParseError::InvalidInput)?;
    let first_pair = pairs.next().ok_or(ParseError::NoMatchFound)?;

    let mut year = String::new();
    let mut month = String::new();
    let mut day = String::new();
    let mut hour = None;
    let mut minute = None;
    let mut second = None;
    let mut millisecond = None;

    let mut components = first_pair.into_inner();
    let date_component = components.next().ok_or(ParseError::NoMatchFound)?;
    let _ = components.next();
    let time_component = components.next();
    for date_inner in date_component.into_inner() {
        match date_inner.as_rule() {
            Rule::year_month_day | Rule::day_month_year | Rule::month_day_year => {
                for part in date_inner.into_inner() {
                    match part.as_rule() {
                        Rule::year => year = part.as_str().to_string(),
                        Rule::month => month = part.as_str().to_string(),
                        Rule::day => day = part.as_str().to_string(),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    if let Some(tc) = time_component {
        for time_inner in tc.into_inner() {
            match time_inner.as_rule() {
                Rule::hour_24 => hour = Some(time_inner.as_str().to_string()),
                Rule::minute => minute = Some(time_inner.as_str().to_string()),
                Rule::second => second = Some(time_inner.as_str().to_string()),
                Rule::millisecond => millisecond = Some(time_inner.as_str().to_string()),
                _ => {}
            }
        }
    }

    if hour.is_none() && minute.is_none() {
        return Ok(format!("{}-{}-{}", year, month, day));
    }

    let mut iso_format = format!("{}-{}-{}T{}", year, month, day, hour.unwrap_or_default());
    if let Some(minute_value) = minute {
        iso_format.push_str(&format!(":{}", minute_value));
    }
    if let Some(second_value) = second {
        iso_format.push_str(&format!(":{}", second_value));
    }
    if let Some(millisecond_value) = millisecond {
        iso_format.push_str(&format!(".{}", millisecond_value));
    }

    Ok(iso_format)
}

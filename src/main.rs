use anyhow::anyhow;
use pest::Parser;
use date_format_parser::*;

fn main() -> anyhow::Result<()> {
    let input_date = "20/12/2000";
    let parsed_date = Grammar::parse(Rule::date, input_date)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    println!("{:?}", parsed_date);

    Ok(())
}


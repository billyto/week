use clap::Parser;
use anyhow::{Context, Ok, Result};
use week::year_week;


/// Simple utility to get week number
#[derive(Parser,Debug)]
#[command(version, about, long_about = None)]
struct Args{
    // Optional date on "%Y-%m-%d",  "%d/%m" or "%d.%m" format, if no date pased, takes the current date.
    #[arg(short, long)]
    date: Option<String>

}
fn main() -> Result<()>{ 

    let args = Args::parse();
    let date = args.date; 

    let week_of_year = year_week(date).with_context(|| format!("Unable to parse date"))?;
    println!("Is weeek {}", week_of_year);
    Ok(())

}





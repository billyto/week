
use chrono::{Datelike, NaiveDate, ParseError, Local};
use clap::Parser;


/// Simple utility to get week number
#[derive(Parser,Debug)]
#[command(version, about, long_about = None)]
struct Args{
    // Optional date on "%Y-%m-%d" format, if no date pased, takes the current date.
    #[arg(short, long)]
    date: Option<String>

}
fn main() {
    let args = Args::parse();
    let date = args.date; 
    let this_week = Local::now().date_naive().iso_week().week();

    let week_of_year = date.map_or(this_week, |day| week(&day).unwrap());

    println!("Is weeek {}", week_of_year);

}

fn week(str_date: &str) -> Result<u32, ParseError> {

   let iso_week = NaiveDate::parse_from_str(str_date, "%Y-%m-%d")?.iso_week();
   Ok(iso_week.week())

}



#[cfg(test)]
mod tests {

    use chrono::NaiveDate;

    use crate::week;

    #[test]
    fn test_parse() {
        
        let parse_from_str = NaiveDate::parse_from_str;
        assert_eq!(parse_from_str("2015-09-05", "%Y-%m-%d"),
               Ok(NaiveDate::from_ymd_opt(2015, 9, 5).unwrap()));

    }

    #[test]
    fn test_week(){
        assert_eq!(week("2023-02-19"), Ok(7));

    }

}
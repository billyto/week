use clap::Parser;
use anyhow::{Context, Ok, Result};
use week::year_week;

/// Simple utility to get the ISO week number (1-53) for any date
///
/// Returns the current week number if no date is provided.
/// Supports multiple date formats for convenience.
#[derive(Parser, Debug)]
#[command(
    name = "week",
    version,
    about = "Get ISO week number for any date",
    long_about = "A simple CLI utility that returns the ISO week number (1-53) for a given date.\n\
                  If no date is provided, returns the current week number.\n\
                  \n\
                  Examples:\n  \
                  week                    # Current week\n  \
                  week -d 19-02-2023     # Week for specific date\n  \
                  week --date 15/03      # Week for date in current year"
)]
struct Args {
    /// Date to get week number for
    ///
    /// Supported formats:
    ///   - DD-MM-YYYY (e.g., 19-02-2023)
    ///   - DD/MM/YYYY (e.g., 19/02/2023)  
    ///   - DD.MM.YYYY (e.g., 19.02.2023)   
    ///   - DD/MM (e.g., 19.02) - uses current year
    ///   - DD/MM (e.g., 19/02) - uses current year
    ///   - DD.MM (e.g., 19.02) - uses current year
    ///
    /// If not provided, returns the current week number
    #[arg(
        short = 'd',
        long = "date",
        value_name = "DATE",
        help = "Date to get week number for (various formats supported)"
    )]
    date: Option<String>,
}
fn main() -> Result<()> {
    let args = Args::parse();
    let date = args.date;


    let week_of_year = year_week(date)
        .with_context(|| "Unable to parse date. Please use format: DD.MM.YYYY, DD-MM-YYYY, DD/MM/YYYY, DD-MM, DD/MM, or DD.MM")?;
    println!("Is weeek {}", week_of_year);
    Ok(())
    
    
    
    
}

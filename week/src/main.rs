use clap::Parser;
use week::year_week;


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

    let week_of_year = year_week(date).unwrap();
    println!("Is weeek {}", week_of_year);

}





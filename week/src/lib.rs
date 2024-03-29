
use chrono::{Datelike, NaiveDate, ParseError, Local};
use regex::Regex;


pub fn year_week(date: Option<String>) ->  Result<u32, ParseError> {

    let this_week = Local::now().date_naive().iso_week().week();
    date.map_or(Ok(this_week), |day| week(&day))

}

#[allow(dead_code)]
fn old_week(str_date: &str) -> Result<u32, ParseError> { //TODO: will remove

    let iso_week = NaiveDate::parse_from_str(str_date, "%Y-%m-%d").
                            or(NaiveDate::parse_from_str(str_date, "%Y/%m/%d"))?.   //support date w/o year (for current)
                            iso_week();
    
    Ok(iso_week.week())
 
 }


fn yearless_week(str_date: &str) -> Result<NaiveDate, ParseError> {

    //check for day/month or day.month
    let re = Regex::new(r"(?<day>[0-9]{1,2})[/|.](?<month>[0-9]{1,2})").unwrap(); 

    let caps = re.captures(str_date).unwrap();
    let day = caps.name("day").unwrap().as_str().parse::<u32>().unwrap();
    let month = caps.name("month").unwrap().as_str().parse::<u32>().unwrap();

    let current_date = chrono::Utc::now();
    let year = current_date.year();

    Ok(NaiveDate::from_ymd_opt(year, month, day).unwrap())
}

 fn week(str_date: &str) -> Result<u32, ParseError> {

    let iso_week = NaiveDate::parse_from_str(str_date, "%d-%m-%Y").
                            or(NaiveDate::parse_from_str(str_date, "%d/%m/%Y")).
                            or_else(|_| yearless_week(str_date) )?.
                            iso_week();
    
    Ok(iso_week.week())
 
 }



#[cfg(test)]
mod tests {


    use crate::{year_week, week};

    #[test]
    fn test_week_slashes(){

        let date_slash = Some("19/02/2023".to_string());
        assert_eq!(year_week(date_slash),Ok(7));

    }

    #[test]
    fn test_week_dashes(){
        let date_dash = Some("19-02-2023".to_string());
        assert_eq!(year_week(date_dash),Ok(7));

    }

    #[test]
    fn test_this_week(){
        
        let date_dash = None;
        let weeks = 1..52; //I don't know when is the reader running this test ¯\_(ツ)_/¯
        let this_week = year_week(date_dash).unwrap();
        assert!(weeks.contains(&this_week));

    }

    #[test]
    fn test_yearless_date(){

        let yearless_date = "02.01";  // This date should always land on week 1
        let week = week(yearless_date).ok().unwrap();

        assert_eq!(1,week)

    }

}
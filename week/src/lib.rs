use chrono::{Datelike, NaiveDate, ParseError};

pub fn week(str_date: &str) -> Result<u32, ParseError> {

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
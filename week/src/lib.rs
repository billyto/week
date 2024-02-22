use chrono::{Datelike, NaiveDate, ParseError};

pub fn week(str_date: &str) -> Result<u32, ParseError> {

    let iso_week = NaiveDate::parse_from_str(str_date, "%Y-%m-%d").
    or(NaiveDate::parse_from_str(str_date, "%Y/%m/%d"))?.iso_week();
    Ok(iso_week.week())
 
 }



#[cfg(test)]
mod tests {

    use crate::week;


    #[test]
    fn test_week_slashes(){
        assert_eq!(week("2023/02/19"), Ok(7));

    }

    #[test]
    fn test_week_dashes(){
        assert_eq!(week("2023-02-19"), Ok(7));

    }

}
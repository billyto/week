use chrono::{Datelike, NaiveDate, ParseError, Local};


pub fn year_week(date: Option<String>) ->  Result<u32, ParseError> {

    let this_week = Local::now().date_naive().iso_week().week();
    date.map_or(Ok(this_week), |day| week(&day))

}


fn week(str_date: &str) -> Result<u32, ParseError> {

    let iso_week = NaiveDate::parse_from_str(str_date, "%Y-%m-%d").
                            or(NaiveDate::parse_from_str(str_date, "%Y/%m/%d"))?.   //support date w/o year (for current)
                            iso_week();
    
    Ok(iso_week.week())
 
 }



#[cfg(test)]
mod tests {

    use crate::year_week;

    #[test]
    fn test_week_slashes(){

        let date_slash = Some("2023/02/19".to_string());
        assert_eq!(year_week(date_slash),Ok(7));

    }

    #[test]
    fn test_week_dashes(){
        let date_dash = Some("2023-02-19".to_string());
        assert_eq!(year_week(date_dash),Ok(7));

    }

    #[test]
    fn test_this_week_(){
        
        let date_dash = None;
        let weeks = 1..52; //I don't know when is the reader running this test ¯\_(ツ)_/¯
        let this_week = year_week(date_dash).unwrap();
        assert!(weeks.contains(&this_week));

    }

}
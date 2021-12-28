use crate::parse_month_word;
use chrono::NaiveDate;

pub fn parse_month_title(s: &str) -> Result<NaiveDate, String> {
    if let Some((month, year)) = s[1..].trim().split_once(" ") {
        if let Ok(year) = year.parse::<i32>() {
            let month = parse_month_word(month)?;
            Ok(NaiveDate::from_ymd(year, month, 1))
        } else {
            Err(format!(
                "Cannot parse as month title '{}' because year is not valid i32",
                s
            ))
        }
    } else {
        Err(format!("Cannot parse as month title '{}'", s))
    }

    // FIXME this should be the best way, but seem to does not work :(
    // DateTime::parse_from_str(&s.replace("#", "1"), "%d %B %Y")
    //     .map_err(|e| format!("Cannot parse as month title '{}' because {:?}", s, e))
    //     .map(|x| x.date().naive_utc())
}

#[cfg(test)]
mod parse_month_title_tests {
    use super::*;
    #[test]
    fn it_should_parse_french_date() {
        assert_eq!(
            parse_month_title("# Janvier 2021"),
            Ok(NaiveDate::from_ymd(2021, 1, 1))
        );
        assert_eq!(
            parse_month_title("# Février 2021"),
            Ok(NaiveDate::from_ymd(2021, 2, 1))
        );
        assert_eq!(
            parse_month_title("# Mars 2021"),
            Ok(NaiveDate::from_ymd(2021, 3, 1))
        );
        assert_eq!(
            parse_month_title("# Avril 2021"),
            Ok(NaiveDate::from_ymd(2021, 4, 1))
        );
        assert_eq!(
            parse_month_title("# Mai 2021"),
            Ok(NaiveDate::from_ymd(2021, 5, 1))
        );
        assert_eq!(
            parse_month_title("# Juin 2021"),
            Ok(NaiveDate::from_ymd(2021, 6, 1))
        );
        assert_eq!(
            parse_month_title("# Juillet 2021"),
            Ok(NaiveDate::from_ymd(2021, 7, 1))
        );
        assert_eq!(
            parse_month_title("# Août 2021"),
            Ok(NaiveDate::from_ymd(2021, 8, 1))
        );
        assert_eq!(
            parse_month_title("# Septembre 2021"),
            Ok(NaiveDate::from_ymd(2021, 9, 1))
        );
        assert_eq!(
            parse_month_title("# Octobre 2021"),
            Ok(NaiveDate::from_ymd(2021, 10, 1))
        );
        assert_eq!(
            parse_month_title("# Novembre 2021"),
            Ok(NaiveDate::from_ymd(2021, 11, 1))
        );
        assert_eq!(
            parse_month_title("# Décembre 2021"),
            Ok(NaiveDate::from_ymd(2021, 12, 1))
        );
    }
    #[test]
    fn it_should_parse_french_date_lowercase() {
        assert_eq!(
            parse_month_title("# janvier 2021"),
            Ok(NaiveDate::from_ymd(2021, 1, 1))
        );
        assert_eq!(
            parse_month_title("# février 2021"),
            Ok(NaiveDate::from_ymd(2021, 2, 1))
        );
        assert_eq!(
            parse_month_title("# mars 2021"),
            Ok(NaiveDate::from_ymd(2021, 3, 1))
        );
        assert_eq!(
            parse_month_title("# avril 2021"),
            Ok(NaiveDate::from_ymd(2021, 4, 1))
        );
        assert_eq!(
            parse_month_title("# mai 2021"),
            Ok(NaiveDate::from_ymd(2021, 5, 1))
        );
        assert_eq!(
            parse_month_title("# juin 2021"),
            Ok(NaiveDate::from_ymd(2021, 6, 1))
        );
        assert_eq!(
            parse_month_title("# juillet 2021"),
            Ok(NaiveDate::from_ymd(2021, 7, 1))
        );
        assert_eq!(
            parse_month_title("# août 2021"),
            Ok(NaiveDate::from_ymd(2021, 8, 1))
        );
        assert_eq!(
            parse_month_title("# septembre 2021"),
            Ok(NaiveDate::from_ymd(2021, 9, 1))
        );
        assert_eq!(
            parse_month_title("# octobre 2021"),
            Ok(NaiveDate::from_ymd(2021, 10, 1))
        );
        assert_eq!(
            parse_month_title("# novembre 2021"),
            Ok(NaiveDate::from_ymd(2021, 11, 1))
        );
        assert_eq!(
            parse_month_title("# décembre 2021"),
            Ok(NaiveDate::from_ymd(2021, 12, 1))
        );
    }
    #[test]
    fn it_should_parse_french_date_no_accent() {
        assert_eq!(
            parse_month_title("# Fevrier 2021"),
            Ok(NaiveDate::from_ymd(2021, 2, 1))
        );
        assert_eq!(
            parse_month_title("# Aout 2021"),
            Ok(NaiveDate::from_ymd(2021, 8, 1))
        );
        assert_eq!(
            parse_month_title("# Decembre 2021"),
            Ok(NaiveDate::from_ymd(2021, 12, 1))
        );
        assert_eq!(
            parse_month_title("# fevrier 2021"),
            Ok(NaiveDate::from_ymd(2021, 2, 1))
        );
        assert_eq!(
            parse_month_title("# aout 2021"),
            Ok(NaiveDate::from_ymd(2021, 8, 1))
        );
        assert_eq!(
            parse_month_title("# decembre 2021"),
            Ok(NaiveDate::from_ymd(2021, 12, 1))
        );
    }

    #[test]
    fn it_should_parse_english_date() {
        assert_eq!(
            parse_month_title("# january 2021"),
            Ok(NaiveDate::from_ymd(2021, 1, 1))
        );
        assert_eq!(
            parse_month_title("# February 2021"),
            Ok(NaiveDate::from_ymd(2021, 2, 1))
        );
        assert_eq!(
            parse_month_title("# March 2021"),
            Ok(NaiveDate::from_ymd(2021, 3, 1))
        );
        assert_eq!(
            parse_month_title("# April 2021"),
            Ok(NaiveDate::from_ymd(2021, 4, 1))
        );
        assert_eq!(
            parse_month_title("# May 2021"),
            Ok(NaiveDate::from_ymd(2021, 5, 1))
        );
        assert_eq!(
            parse_month_title("# June 2021"),
            Ok(NaiveDate::from_ymd(2021, 6, 1))
        );
        assert_eq!(
            parse_month_title("# July 2021"),
            Ok(NaiveDate::from_ymd(2021, 7, 1))
        );
        assert_eq!(
            parse_month_title("# August 2021"),
            Ok(NaiveDate::from_ymd(2021, 8, 1))
        );
        assert_eq!(
            parse_month_title("# September 2021"),
            Ok(NaiveDate::from_ymd(2021, 9, 1))
        );
        assert_eq!(
            parse_month_title("# October 2021"),
            Ok(NaiveDate::from_ymd(2021, 10, 1))
        );
        assert_eq!(
            parse_month_title("# November 2021"),
            Ok(NaiveDate::from_ymd(2021, 11, 1))
        );
        assert_eq!(
            parse_month_title("# December 2021"),
            Ok(NaiveDate::from_ymd(2021, 12, 1))
        );
    }
    #[test]
    fn it_should_parse_english_date_lowercase() {
        assert_eq!(
            parse_month_title("# january 2021"),
            Ok(NaiveDate::from_ymd(2021, 1, 1))
        );
        assert_eq!(
            parse_month_title("# february 2021"),
            Ok(NaiveDate::from_ymd(2021, 2, 1))
        );
        assert_eq!(
            parse_month_title("# march 2021"),
            Ok(NaiveDate::from_ymd(2021, 3, 1))
        );
        assert_eq!(
            parse_month_title("# april 2021"),
            Ok(NaiveDate::from_ymd(2021, 4, 1))
        );
        assert_eq!(
            parse_month_title("# may 2021"),
            Ok(NaiveDate::from_ymd(2021, 5, 1))
        );
        assert_eq!(
            parse_month_title("# june 2021"),
            Ok(NaiveDate::from_ymd(2021, 6, 1))
        );
        assert_eq!(
            parse_month_title("# july 2021"),
            Ok(NaiveDate::from_ymd(2021, 7, 1))
        );
        assert_eq!(
            parse_month_title("# august 2021"),
            Ok(NaiveDate::from_ymd(2021, 8, 1))
        );
        assert_eq!(
            parse_month_title("# september 2021"),
            Ok(NaiveDate::from_ymd(2021, 9, 1))
        );
        assert_eq!(
            parse_month_title("# october 2021"),
            Ok(NaiveDate::from_ymd(2021, 10, 1))
        );
        assert_eq!(
            parse_month_title("# november 2021"),
            Ok(NaiveDate::from_ymd(2021, 11, 1))
        );
        assert_eq!(
            parse_month_title("# december 2021"),
            Ok(NaiveDate::from_ymd(2021, 12, 1))
        );
    }
}

//! A [nom](https://crates.io/crates/nom)-based Recursive Descent Parser and
//! renderer for human-friendly date/time input as supported by Emacs'
//! [Org-mode](https://orgmode.org/manual/The-date_002ftime-prompt.html).
mod error;
mod parser;
mod renderer;

pub use error::{OrgModeDateTimeError, Result};
pub use nom::IResult;
pub use parser::{parse, Absolute, AbsoluteTime, DateTimeSpec, Meridiem, Relative, RelativeTime};
pub use renderer::{render, RenderedSpec};

#[cfg(test)]
mod test_roundtrip {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use crate::parser::Relative;

    use super::*;

    #[test]
    fn dashed_date() {
        let (input, spec) = parse("3-2-5").unwrap();
        assert_eq!(input, "");
        let unused: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        assert_eq!(
            render(unused, unused, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2003, 2, 5))
        );
    }

    #[test]
    fn test_slashed_date() {
        let (input, spec) = parse("2/5/3").unwrap();
        assert_eq!(input, "");
        let unused: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        assert_eq!(
            render(unused, unused, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2003, 2, 5))
        );
    }

    #[test]
    fn test_day_of_month() {
        let (input, spec) = parse("14").unwrap();
        assert_eq!(input, "");
        let now = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused = now;
        assert_eq!(
            render(unused, now, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2006, 6, 14))
        );
    }

    #[test]
    fn test_day_month() {
        let (input, spec) = parse("12").unwrap();
        assert_eq!(input, "");
        let default: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused = default;
        assert_eq!(
            render(default, unused, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2006, 7, 12))
        );
    }

    #[test]
    fn test_day_slash_month() {
        let (input, spec) = parse("2/5").unwrap();
        assert_eq!(input, "");
        let default: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused = default;
        assert_eq!(
            render(default, unused, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2007, 2, 5))
        );
    }

    #[test]
    fn test_week_day() {
        let (input, spec) = parse("Fri").unwrap();
        assert_eq!(input, "");
        let default: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused = default;
        assert_eq!(
            render(default, unused, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2006, 6, 16))
        );
    }

    #[test]
    fn test_month_day() {
        let (input, spec) = parse("sep 15").unwrap();
        assert_eq!(input, "");
        let default: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused = default;
        assert_eq!(
            render(default, unused, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2006, 9, 15))
        );

        let (input, spec) = parse("feb 15").unwrap();
        assert_eq!(input, "");
        let default: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused = default;
        assert_eq!(
            render(default, unused, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2007, 2, 15))
        );
    }

    #[test]
    fn test_month_day_year() {
        let (input, spec) = parse("sep 12 9").unwrap();
        assert_eq!(input, "");
        let default: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused = default;
        assert_eq!(
            render(default, unused, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2009, 9, 12))
        );
    }

    #[test]
    fn test_hour_minute() {
        let (input, spec) = parse("12:45").unwrap();
        assert_eq!(input, "");
        let default: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused = default;
        assert_eq!(
            render(default, unused, spec).unwrap(),
            RenderedSpec::DateTime(NaiveDate::from_ymd(2006, 6, 13).and_hms(12, 45, 0))
        );
    }

    #[test]
    fn test_day_month_hour_minute() {
        let (input, spec) = parse("22 sept 0:34").unwrap();
        assert_eq!(input, "");
        let default: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused = default;
        assert_eq!(
            render(default, unused, spec).unwrap(),
            RenderedSpec::DateTime(NaiveDate::from_ymd(2006, 9, 22).and_hms(0, 34, 0))
        );
    }

    #[test]
    fn test_week_number() {
        let (input, spec) = parse("w4").unwrap();
        assert_eq!(input, "");
        let unused: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        assert_eq!(render(unused, unused, spec).unwrap(), RenderedSpec::Week(4));
    }

    #[test]
    fn test_week_date() {
        let (input, spec) = parse("2012 w4 fri").unwrap();
        assert_eq!(input, "");
        let default: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused = default;
        assert_eq!(
            render(default, unused, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2012, 1, 27))
        );

        let (input, spec) = parse("2012-w04-5").unwrap();
        assert_eq!(input, "");
        let default: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused = default;
        assert_eq!(
            render(default, unused, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2012, 1, 27))
        );
    }

    #[test]
    fn test_plus_zero() {
        let (input, spec) = parse("+0").unwrap();
        assert_eq!(input, "");
        let default: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let now: NaiveDateTime = default;
        assert_eq!(
            render(default, now, spec).unwrap(),
            RenderedSpec::Date(now.date())
        );
    }

    #[test]
    fn test_dot() {
        let (input, spec) = parse(".").unwrap();
        assert_eq!(input, "");
        let default: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let now: NaiveDateTime = default;
        assert_eq!(
            render(default, now, spec).unwrap(),
            RenderedSpec::Date(now.date())
        );
    }

    #[test]
    fn test_plus_hours() {
        let (input, spec) = parse("+2h").unwrap();
        assert_eq!(input, "");
        let now: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused: NaiveDateTime = now;
        assert_eq!(
            render(unused, now, spec).unwrap(),
            RenderedSpec::DateTime(NaiveDate::from_ymd(2006, 6, 13).and_hms(2, 0, 0))
        );
    }

    #[test]
    fn test_plus_days() {
        let (input, spec) = parse("+4d").unwrap();
        assert_eq!(input, "");
        let now: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused: NaiveDateTime = now;
        assert_eq!(
            render(unused, now, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2006, 6, 17))
        );
    }

    #[test]
    fn test_plus_weeks() {
        let (input, spec) = parse("+4w").unwrap();
        assert_eq!(input, "");
        let now: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused: NaiveDateTime = now;
        assert_eq!(
            render(unused, now, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2006, 7, 11))
        );
    }

    #[test]
    fn test_plus_months() {
        let (input, spec) = parse("+3m").unwrap();
        assert_eq!(input, "");
        let now: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused: NaiveDateTime = now;
        assert_eq!(
            render(unused, now, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2006, 9, 13))
        );
    }

    #[test]
    fn test_minus_months_impossible() {
        let (input, spec) = parse("-4m").unwrap();
        assert_eq!(input, "");
        let now: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 30).and_hms(0, 0, 0);
        let unused: NaiveDateTime = now;
        assert!(matches!(
            render(unused, now, spec),
            Err(OrgModeDateTimeError::UnrepresentablePastRelativeDate(
                Relative {
                    hours: None,
                    days: None,
                    weeks: None,
                    weekdays: None,
                    months: Some(4),
                    years: None,
                }
            ))
        ));
    }

    #[test]
    fn test_minus_months() {
        let (input, spec) = parse("-3m").unwrap();
        assert_eq!(input, "");
        let now: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused: NaiveDateTime = now;
        assert_eq!(
            render(unused, now, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2006, 3, 13))
        );
    }

    #[test]
    fn test_plus_years() {
        let (input, spec) = parse("+3y").unwrap();
        assert_eq!(input, "");
        let now: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused: NaiveDateTime = now;
        assert_eq!(
            render(unused, now, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2009, 6, 13))
        );
    }

    #[test]
    fn test_default_plus_days() {
        let (input, spec) = parse("++5").unwrap();
        assert_eq!(input, "");
        let default: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused: NaiveDateTime = default;
        assert_eq!(
            render(default, unused, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2006, 6, 18))
        );
    }

    #[test]
    fn test_default_minus_days() {
        let (input, spec) = parse("--5").unwrap();
        assert_eq!(input, "");
        let default: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused: NaiveDateTime = default;
        assert_eq!(
            render(default, unused, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2006, 6, 8))
        );
    }

    #[test]
    fn test_plus_relative_weekdays() {
        let (input, spec) = parse("+2tue").unwrap();
        assert_eq!(input, "");
        let now: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused: NaiveDateTime = now;
        assert_eq!(
            render(unused, now, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2006, 6, 27))
        );
    }

    #[test]
    fn test_minus_relative_weekdays() {
        let (input, spec) = parse("-wed").unwrap();
        assert_eq!(input, "");
        let now: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused: NaiveDateTime = now;
        assert_eq!(
            render(unused, now, spec).unwrap(),
            RenderedSpec::Date(NaiveDate::from_ymd(2006, 6, 7))
        );
    }

    #[test]
    fn test_time_range() {
        let (input, spec) = parse("11am-1:15pm").unwrap();
        assert_eq!(input, "");
        let now: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused: NaiveDateTime = now;
        assert_eq!(
            render(unused, now, spec).unwrap(),
            RenderedSpec::TimeRange(
                NaiveTime::from_hms(11, 0, 0),
                NaiveTime::from_hms(13, 15, 0)
            )
        );
    }

    #[test]
    fn test_time_range_double_dash() {
        let (input, spec) = parse("11am--1:15pm").unwrap();
        assert_eq!(input, "");
        let now: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused: NaiveDateTime = now;
        assert_eq!(
            render(unused, now, spec).unwrap(),
            RenderedSpec::TimeRange(
                NaiveTime::from_hms(11, 0, 0),
                NaiveTime::from_hms(13, 15, 0)
            )
        );
    }

    #[test]
    fn test_time_duration() {
        let (input, spec) = parse("11am+2:15").unwrap();
        assert_eq!(input, "");
        let now: NaiveDateTime = NaiveDate::from_ymd(2006, 6, 13).and_hms(0, 0, 0);
        let unused: NaiveDateTime = now;
        assert_eq!(
            render(unused, now, spec).unwrap(),
            RenderedSpec::TimeRange(
                NaiveTime::from_hms(11, 0, 0),
                NaiveTime::from_hms(13, 15, 0)
            )
        );
    }
}

use chrono::{Month, Weekday};
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{digit1, space1};
use nom::combinator::opt;
use nom::{branch::alt, IResult};

#[derive(Debug, PartialEq, Eq)]
pub enum Meridiem {
    AM,
    PM,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Absolute {
    pub year: Option<u32>,
    pub month: Option<u32>,
    pub day: Option<u32>,
    pub week: Option<u32>,
    pub weekday: Option<u32>,
    pub hour: Option<u32>,
    pub minute: Option<u32>,
    pub meridiem: Option<Meridiem>,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct AbsoluteTime {
    pub hour: Option<u32>,
    pub minute: Option<u32>,
    pub meridiem: Option<Meridiem>,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Relative {
    pub hours: Option<u32>,
    pub days: Option<u32>,
    pub weeks: Option<u32>,
    pub weekdays: Option<u32>,
    pub months: Option<u32>,
    pub years: Option<u32>,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct RelativeTime {
    pub hours: Option<u32>,
    pub minutes: Option<u32>,
}

// https://orgmode.org/manual/The-date_002ftime-prompt.html
#[derive(Debug, PartialEq, Eq)]
pub enum DateTimeSpec {
    Absolute(Absolute),
    NowRelativeFuture(Relative),
    NowRelativePast(Relative),
    DefaultRelativeFuture(Relative),
    DefaultRelativePast(Relative),
    TimeRangeAbsoluteStartAbsoluteEnd(AbsoluteTime, AbsoluteTime),
    TimeRangeAbsoluteStartRelativeEnd(AbsoluteTime, RelativeTime),
}

fn monday(input: &str) -> IResult<&str, Weekday> {
    let (input, _) = alt((
        tag_no_case("monday"),
        tag_no_case("monda"),
        tag_no_case("mond"),
        tag_no_case("mon"),
    ))(input)?;
    Ok((input, Weekday::Mon))
}

fn tuesday(input: &str) -> IResult<&str, Weekday> {
    let (input, _) = alt((
        tag_no_case("tuesday"),
        tag_no_case("tuesda"),
        tag_no_case("tuesd"),
        tag_no_case("tue"),
    ))(input)?;
    Ok((input, Weekday::Tue))
}

fn wednesday(input: &str) -> IResult<&str, Weekday> {
    let (input, _) = alt((
        tag_no_case("wednesday"),
        tag_no_case("wednesda"),
        tag_no_case("wednesd"),
        tag_no_case("wednes"),
        tag_no_case("wedne"),
        tag_no_case("wedn"),
        tag_no_case("wed"),
    ))(input)?;
    Ok((input, Weekday::Wed))
}

fn thursday(input: &str) -> IResult<&str, Weekday> {
    let (input, _) = alt((
        tag_no_case("thursday"),
        tag_no_case("thursda"),
        tag_no_case("thursd"),
        tag_no_case("thurs"),
        tag_no_case("thur"),
        tag_no_case("thu"),
    ))(input)?;
    Ok((input, Weekday::Thu))
}

fn friday(input: &str) -> IResult<&str, Weekday> {
    let (input, _) = alt((
        tag_no_case("friday"),
        tag_no_case("frida"),
        tag_no_case("frid"),
        tag_no_case("fri"),
    ))(input)?;
    Ok((input, Weekday::Fri))
}

fn saturday(input: &str) -> IResult<&str, Weekday> {
    let (input, _) = alt((
        tag_no_case("saturday"),
        tag_no_case("saturda"),
        tag_no_case("saturd"),
        tag_no_case("satur"),
        tag_no_case("satu"),
        tag_no_case("sat"),
    ))(input)?;
    Ok((input, Weekday::Sat))
}

fn sunday(input: &str) -> IResult<&str, Weekday> {
    let (input, _) = alt((
        tag_no_case("sunday"),
        tag_no_case("sunda"),
        tag_no_case("sund"),
        tag_no_case("sun"),
    ))(input)?;
    Ok((input, Weekday::Sun))
}

fn january(input: &str) -> IResult<&str, Month> {
    let (input, _) = alt((
        tag_no_case("january"),
        tag_no_case("januar"),
        tag_no_case("janu"),
        tag_no_case("jan"),
    ))(input)?;
    Ok((input, Month::January))
}

fn february(input: &str) -> IResult<&str, Month> {
    let (input, _) = alt((
        tag_no_case("february"),
        tag_no_case("februar"),
        tag_no_case("februa"),
        tag_no_case("febru"),
        tag_no_case("febr"),
        tag_no_case("feb"),
    ))(input)?;
    Ok((input, Month::February))
}

fn march(input: &str) -> IResult<&str, Month> {
    let (input, _) = alt((
        tag_no_case("march"),
        tag_no_case("marc"),
        tag_no_case("mar"),
    ))(input)?;
    Ok((input, Month::March))
}

fn april(input: &str) -> IResult<&str, Month> {
    let (input, _) = alt((
        tag_no_case("april"),
        tag_no_case("apri"),
        tag_no_case("apr"),
    ))(input)?;
    Ok((input, Month::April))
}

fn may(input: &str) -> IResult<&str, Month> {
    let (input, _) = tag_no_case("may")(input)?;
    Ok((input, Month::May))
}

fn june(input: &str) -> IResult<&str, Month> {
    let (input, _) = alt((tag_no_case("june"), tag_no_case("jun")))(input)?;
    Ok((input, Month::June))
}

fn july(input: &str) -> IResult<&str, Month> {
    let (input, _) = alt((tag_no_case("july"), tag_no_case("jul")))(input)?;
    Ok((input, Month::July))
}

fn august(input: &str) -> IResult<&str, Month> {
    let (input, _) = alt((
        tag_no_case("august"),
        tag_no_case("augus"),
        tag_no_case("augu"),
        tag_no_case("aug"),
    ))(input)?;
    Ok((input, Month::August))
}

fn september(input: &str) -> IResult<&str, Month> {
    let (input, _) = alt((
        tag_no_case("september"),
        tag_no_case("septembe"),
        tag_no_case("septemb"),
        tag_no_case("septem"),
        tag_no_case("septe"),
        tag_no_case("sept"),
        tag_no_case("sep"),
    ))(input)?;
    Ok((input, Month::September))
}

fn november(input: &str) -> IResult<&str, Month> {
    let (input, _) = alt((
        tag_no_case("november"),
        tag_no_case("novembe"),
        tag_no_case("novemb"),
        tag_no_case("novem"),
        tag_no_case("nove"),
        tag_no_case("nov"),
    ))(input)?;
    Ok((input, Month::November))
}

fn october(input: &str) -> IResult<&str, Month> {
    let (input, _) = alt((
        tag_no_case("october"),
        tag_no_case("octobe"),
        tag_no_case("octob"),
        tag_no_case("octo"),
        tag_no_case("oct"),
    ))(input)?;
    Ok((input, Month::October))
}

fn december(input: &str) -> IResult<&str, Month> {
    let (input, _) = alt((
        tag_no_case("december"),
        tag_no_case("decembe"),
        tag_no_case("decemb"),
        tag_no_case("decem"),
        tag_no_case("dece"),
        tag_no_case("dec"),
    ))(input)?;
    Ok((input, Month::December))
}

fn month(input: &str) -> IResult<&str, Month> {
    alt((
        january, february, march, april, may, june, july, august, september, october, november,
        december,
    ))(input)
}

fn month_as_number(input: &str) -> IResult<&str, u32> {
    let (input, month_enum) = month(input)?;
    Ok((input, month_enum.number_from_month()))
}

fn number(input: &str) -> IResult<&str, u32> {
    let (input, year_str) = digit1(input)?;
    let year: u32 = year_str.parse().unwrap();
    Ok((input, year))
}

fn parse_dashed_date(input: &str) -> IResult<&str, DateTimeSpec> {
    let (input, year) = number(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, month) = number(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, day) = number(input)?;
    let result = Absolute {
        year: Some(year),
        month: Some(month),
        day: Some(day),
        ..Default::default()
    };
    Ok((input, DateTimeSpec::Absolute(result)))
}

fn slashed_date_month_day_year(input: &str) -> IResult<&str, Absolute> {
    let (input, month) = number(input)?;
    let (input, _) = tag("/")(input)?;
    let (input, day) = number(input)?;
    let (input, _) = tag("/")(input)?;
    let (input, year) = number(input)?;
    Ok((
        input,
        Absolute {
            year: Some(year),
            month: Some(month),
            day: Some(day),
            ..Default::default()
        },
    ))
}

fn slashed_date_month_day(input: &str) -> IResult<&str, Absolute> {
    let (input, month) = number(input)?;
    let (input, _) = tag("/")(input)?;
    let (input, day) = number(input)?;
    Ok((
        input,
        Absolute {
            month: Some(month),
            day: Some(day),
            ..Default::default()
        },
    ))
}

fn parse_slashed_date(input: &str) -> IResult<&str, DateTimeSpec> {
    let (input, result) = alt((slashed_date_month_day_year, slashed_date_month_day))(input)?;
    Ok((input, DateTimeSpec::Absolute(result)))
}

fn iso_week_number(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("w")(input)?;
    let (input, week_str) = digit1(input)?;
    let week: u32 = week_str.parse().unwrap();
    Ok((input, week))
}

fn weekday(input: &str) -> IResult<&str, u32> {
    let (input, output) = alt((
        monday, tuesday, wednesday, thursday, friday, saturday, sunday,
    ))(input)?;
    let numeric_weekday = output.number_from_monday();
    Ok((input, numeric_weekday))
}

fn numeric_weekday(input: &str) -> IResult<&str, u32> {
    let (input, weekday_str) = digit1(input)?;
    let weekday: u32 = weekday_str.parse().unwrap();
    Ok((input, weekday))
}

fn parse_weekday(input: &str) -> IResult<&str, DateTimeSpec> {
    let (input, wday) = weekday(input)?;
    let result = Absolute {
        weekday: Some(wday),
        ..Default::default()
    };

    Ok((input, DateTimeSpec::Absolute(result)))
}

fn iso_week_day(input: &str) -> IResult<&str, u32> {
    alt((weekday, numeric_weekday))(input)
}

fn parse_iso_date(input: &str) -> IResult<&str, DateTimeSpec> {
    let (input, year) = number(input)?;
    let (input, _) = alt((tag("-"), space1))(input)?;
    let (input, week) = iso_week_number(input)?;
    let (input, _) = alt((tag("-"), space1))(input)?;
    let (input, weekday) = iso_week_day(input)?;
    let result = Absolute {
        year: Some(year),
        week: Some(week),
        weekday: Some(weekday),
        ..Default::default()
    };
    Ok((input, DateTimeSpec::Absolute(result)))
}

fn parse_month_day_year(input: &str) -> IResult<&str, Absolute> {
    let (input, month) = month_as_number(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, day) = number(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, year) = number(input)?;
    Ok((
        input,
        Absolute {
            month: Some(month),
            day: Some(day),
            year: Some(year),
            ..Default::default()
        },
    ))
}

fn parse_month_day(input: &str) -> IResult<&str, Absolute> {
    let (input, month) = month_as_number(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, day) = number(input)?;
    Ok((
        input,
        Absolute {
            month: Some(month),
            day: Some(day),
            ..Default::default()
        },
    ))
}

fn parse_month_day_optional_year(input: &str) -> IResult<&str, DateTimeSpec> {
    let (input, result) = alt((parse_month_day_year, parse_month_day))(input)?;
    Ok((input, DateTimeSpec::Absolute(result)))
}

pub fn hour_minute_meridiem(input: &str) -> IResult<&str, AbsoluteTime> {
    let (input, hour) = number(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, minute) = number(input)?;
    let (input, meridiem) = meridiem(input)?;
    Ok((
        input,
        AbsoluteTime {
            hour: Some(hour),
            minute: Some(minute),
            meridiem: Some(meridiem),
        },
    ))
}

pub fn hour_meridiem(input: &str) -> IResult<&str, AbsoluteTime> {
    let (input, hour) = number(input)?;
    let (input, meridiem) = meridiem(input)?;
    Ok((
        input,
        AbsoluteTime {
            hour: Some(hour),
            meridiem: Some(meridiem),
            ..Default::default()
        },
    ))
}

pub fn hour_minute(input: &str) -> IResult<&str, AbsoluteTime> {
    let (input, hour) = number(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, minute) = number(input)?;
    Ok((
        input,
        AbsoluteTime {
            hour: Some(hour),
            minute: Some(minute),
            ..Default::default()
        },
    ))
}

pub fn parse_day_month_hour_minute(input: &str) -> IResult<&str, Absolute> {
    let (input, day) = number(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, month) = month_as_number(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, hour) = number(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, minute) = number(input)?;
    Ok((
        input,
        Absolute {
            month: Some(month),
            day: Some(day),
            hour: Some(hour),
            minute: Some(minute),
            ..Default::default()
        },
    ))
}

pub fn parse_day_of_month(input: &str) -> IResult<&str, Absolute> {
    let (input, day) = number(input)?;
    Ok((
        input,
        Absolute {
            day: Some(day),
            ..Default::default()
        },
    ))
}

fn parse_time_as_absolute(input: &str) -> IResult<&str, Absolute> {
    let (input, time) = parse_time(input)?;
    Ok((
        input,
        Absolute {
            hour: time.hour,
            minute: time.minute,
            meridiem: time.meridiem,
            ..Default::default()
        },
    ))
}

pub fn parse_day_optional_month_optional_hour_minute(input: &str) -> IResult<&str, DateTimeSpec> {
    let (input, result) = alt((
        parse_day_month_hour_minute,
        parse_time_as_absolute,
        parse_day_of_month,
    ))(input)?;
    Ok((input, DateTimeSpec::Absolute(result)))
}

fn parse_iso_week_number(input: &str) -> IResult<&str, DateTimeSpec> {
    let (input, week) = iso_week_number(input)?;
    let result = Absolute {
        week: Some(week),
        ..Default::default()
    };
    Ok((input, DateTimeSpec::Absolute(result)))
}

fn parse_relative_hours(input: &str) -> IResult<&str, Relative> {
    let (input, hours) = number(input)?;
    let (input, _) = tag_no_case("h")(input)?;
    Ok((
        input,
        Relative {
            hours: Some(hours),
            ..Default::default()
        },
    ))
}

fn parse_relative_days(input: &str) -> IResult<&str, Relative> {
    let (input, days) = number(input)?;
    let (input, _) = tag_no_case("d")(input)?;
    Ok((
        input,
        Relative {
            days: Some(days),
            ..Default::default()
        },
    ))
}

fn parse_relative_implied_days(input: &str) -> IResult<&str, Relative> {
    let (input, days) = number(input)?;
    Ok((
        input,
        Relative {
            days: Some(days),
            ..Default::default()
        },
    ))
}

fn parse_relative_weeks(input: &str) -> IResult<&str, Relative> {
    let (input, weeks) = number(input)?;
    let (input, _) = tag_no_case("w")(input)?;
    Ok((
        input,
        Relative {
            weeks: Some(weeks),
            ..Default::default()
        },
    ))
}

fn parse_relative_months(input: &str) -> IResult<&str, Relative> {
    let (input, months) = number(input)?;
    let (input, _) = tag_no_case("m")(input)?;
    Ok((
        input,
        Relative {
            months: Some(months),
            ..Default::default()
        },
    ))
}

fn parse_relative_years(input: &str) -> IResult<&str, Relative> {
    let (input, years) = number(input)?;
    let (input, _) = tag_no_case("y")(input)?;
    Ok((
        input,
        Relative {
            years: Some(years),
            ..Default::default()
        },
    ))
}

fn parse_relative_number_weekdays(input: &str) -> IResult<&str, Relative> {
    let (input, weeks) = number(input)?;
    let (input, weekdays) = weekday(input)?;
    Ok((
        input,
        Relative {
            weeks: Some(weeks),
            weekdays: Some(weekdays),
            ..Default::default()
        },
    ))
}

fn parse_relative_implied_one_weekday(input: &str) -> IResult<&str, Relative> {
    let (input, weekdays) = weekday(input)?;
    Ok((
        input,
        Relative {
            weekdays: Some(weekdays),
            ..Default::default()
        },
    ))
}

fn parse_relative_weekdays(input: &str) -> IResult<&str, Relative> {
    alt((
        parse_relative_number_weekdays,
        parse_relative_implied_one_weekday,
    ))(input)
}

fn parse_relative(input: &str) -> IResult<&str, Relative> {
    let (input, relative) = alt((
        parse_relative_hours,
        parse_relative_days,
        parse_relative_weeks,
        parse_relative_months,
        parse_relative_years,
        parse_relative_weekdays,
        parse_relative_implied_days,
    ))(input)?;
    Ok((input, relative))
}

fn dot_plus_relative(input: &str) -> IResult<&str, DateTimeSpec> {
    let (input, _) = tag(".")(input)?;
    let (input, _) = tag("+")(input)?;
    let (input, relative) = parse_relative(input)?;
    Ok((input, DateTimeSpec::NowRelativeFuture(relative)))
}

fn plus_relative(input: &str) -> IResult<&str, DateTimeSpec> {
    let (input, _) = tag("+")(input)?;
    let (input, relative) = parse_relative(input)?;
    Ok((input, DateTimeSpec::NowRelativeFuture(relative)))
}

fn dot_minus_relative(input: &str) -> IResult<&str, DateTimeSpec> {
    let (input, _) = tag(".")(input)?;
    let (input, _) = tag("+")(input)?;
    let (input, relative) = parse_relative(input)?;
    Ok((input, DateTimeSpec::NowRelativePast(relative)))
}

fn minus_relative(input: &str) -> IResult<&str, DateTimeSpec> {
    let (input, _) = tag("-")(input)?;
    let (input, relative) = parse_relative(input)?;
    Ok((input, DateTimeSpec::NowRelativePast(relative)))
}

fn dot_relative(input: &str) -> IResult<&str, DateTimeSpec> {
    let (input, _) = tag(".")(input)?;
    let relative = Relative {
        ..Default::default()
    };
    Ok((input, DateTimeSpec::NowRelativeFuture(relative)))
}

fn plus_plus_relative(input: &str) -> IResult<&str, DateTimeSpec> {
    let (input, _) = tag("++")(input)?;
    let (input, relative) = parse_relative(input)?;
    Ok((input, DateTimeSpec::DefaultRelativeFuture(relative)))
}

fn minus_minus_relative(input: &str) -> IResult<&str, DateTimeSpec> {
    let (input, _) = tag("--")(input)?;
    let (input, relative) = parse_relative(input)?;
    Ok((input, DateTimeSpec::DefaultRelativePast(relative)))
}

fn ante_meridiem(input: &str) -> IResult<&str, Meridiem> {
    let (input, _) = tag_no_case("am")(input)?;
    Ok((input, Meridiem::AM))
}

fn poste_meridiem(input: &str) -> IResult<&str, Meridiem> {
    let (input, _) = tag_no_case("pm")(input)?;
    Ok((input, Meridiem::PM))
}

fn meridiem(input: &str) -> IResult<&str, Meridiem> {
    alt((ante_meridiem, poste_meridiem))(input)
}

fn parse_time(input: &str) -> IResult<&str, AbsoluteTime> {
    alt((hour_minute_meridiem, hour_meridiem, hour_minute))(input)
}

fn parse_time_range(input: &str) -> IResult<&str, DateTimeSpec> {
    let (input, start) = parse_time(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, _) = opt(tag("-"))(input)?;
    let (input, end) = parse_time(input)?;
    Ok((
        input,
        DateTimeSpec::TimeRangeAbsoluteStartAbsoluteEnd(start, end),
    ))
}

fn parse_duration(input: &str) -> IResult<&str, RelativeTime> {
    let (input, hours) = number(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, minutes) = number(input)?;
    Ok((
        input,
        RelativeTime {
            hours: Some(hours),
            minutes: Some(minutes),
            ..Default::default()
        },
    ))
}

fn parse_time_duration(input: &str) -> IResult<&str, DateTimeSpec> {
    let (input, start) = parse_time(input)?;
    let (input, _) = tag("+")(input)?;
    let (input, duration) = parse_duration(input)?;
    Ok((
        input,
        DateTimeSpec::TimeRangeAbsoluteStartRelativeEnd(start, duration),
    ))
}

pub fn parse(input: &str) -> IResult<&str, DateTimeSpec> {
    alt((
        parse_dashed_date,
        parse_slashed_date,
        parse_iso_date,
        parse_weekday,
        parse_month_day_optional_year,
        parse_time_duration,
        parse_time_range,
        parse_day_optional_month_optional_hour_minute,
        parse_iso_week_number,
        dot_plus_relative,
        dot_minus_relative,
        dot_relative,
        plus_relative,
        minus_relative,
        plus_plus_relative,
        minus_minus_relative,
    ))(input)
}

#[cfg(test)]
mod test_parsers_bottom_up {
    use super::*;

    #[test]
    fn test_weekday_terminals() {
        assert_eq!(monday("monday").unwrap(), ("", Weekday::Mon));
        assert_eq!(tuesday("tuesday").unwrap(), ("", Weekday::Tue));
        assert_eq!(wednesday("wednesday").unwrap(), ("", Weekday::Wed));
        assert_eq!(thursday("thursday").unwrap(), ("", Weekday::Thu));
        assert_eq!(friday("friday").unwrap(), ("", Weekday::Fri));
        assert_eq!(saturday("saturday").unwrap(), ("", Weekday::Sat));
        assert_eq!(sunday("sunday").unwrap(), ("", Weekday::Sun));
    }

    #[test]
    fn test_weekday_nonterminal() {
        assert_eq!(
            weekday("mon").unwrap(),
            ("", Weekday::Mon.number_from_monday())
        );
        assert_eq!(
            weekday("tue").unwrap(),
            ("", Weekday::Tue.number_from_monday())
        );
        assert_eq!(
            weekday("wed").unwrap(),
            ("", Weekday::Wed.number_from_monday())
        );
        assert_eq!(
            weekday("thu").unwrap(),
            ("", Weekday::Thu.number_from_monday())
        );
        assert_eq!(
            weekday("fri").unwrap(),
            ("", Weekday::Fri.number_from_monday())
        );
        assert_eq!(
            weekday("sat").unwrap(),
            ("", Weekday::Sat.number_from_monday())
        );
        assert_eq!(
            weekday("sun").unwrap(),
            ("", Weekday::Sun.number_from_monday())
        );
    }

    #[test]
    fn test_month_terminals() {
        assert_eq!(january("january").unwrap(), ("", Month::January));
        assert_eq!(february("february").unwrap(), ("", Month::February));
        assert_eq!(march("march").unwrap(), ("", Month::March));
        assert_eq!(april("april").unwrap(), ("", Month::April));
        assert_eq!(may("may").unwrap(), ("", Month::May));
        assert_eq!(june("june").unwrap(), ("", Month::June));
        assert_eq!(july("july").unwrap(), ("", Month::July));
        assert_eq!(august("august").unwrap(), ("", Month::August));
        assert_eq!(september("september").unwrap(), ("", Month::September));
        assert_eq!(october("october").unwrap(), ("", Month::October));
        assert_eq!(november("november").unwrap(), ("", Month::November));
        assert_eq!(december("december").unwrap(), ("", Month::December));
    }

    #[test]
    fn test_month_nonterminal() {
        assert_eq!(month("jan").unwrap(), ("", Month::January));
        assert_eq!(month("feb").unwrap(), ("", Month::February));
        assert_eq!(month("mar").unwrap(), ("", Month::March));
        assert_eq!(month("apr").unwrap(), ("", Month::April));
        assert_eq!(month("may").unwrap(), ("", Month::May));
        assert_eq!(month("jun").unwrap(), ("", Month::June));
        assert_eq!(month("jul").unwrap(), ("", Month::July));
        assert_eq!(month("aug").unwrap(), ("", Month::August));
        assert_eq!(month("sep").unwrap(), ("", Month::September));
        assert_eq!(month("oct").unwrap(), ("", Month::October));
        assert_eq!(month("nov").unwrap(), ("", Month::November));
        assert_eq!(month("dec").unwrap(), ("", Month::December));
    }

    #[test]
    fn test_dashed_date() {
        assert_eq!(
            parse_dashed_date("3-2-5").unwrap(),
            (
                "",
                DateTimeSpec::Absolute(Absolute {
                    year: Some(3),
                    month: Some(2),
                    day: Some(5),
                    ..Default::default()
                })
            )
        );
    }

    #[test]
    fn test_slashed_date() {
        assert_eq!(
            parse_slashed_date("2/5/3").unwrap(),
            (
                "",
                DateTimeSpec::Absolute(Absolute {
                    year: Some(3),
                    month: Some(2),
                    day: Some(5),
                    ..Default::default()
                })
            )
        );
    }

    #[test]
    fn test_iso_date() {
        assert_eq!(
            parse_iso_date("2012 w4 fri").unwrap(),
            (
                "",
                DateTimeSpec::Absolute(Absolute {
                    year: Some(2012),
                    week: Some(4),
                    weekday: Some(5),
                    ..Default::default()
                })
            )
        );

        assert_eq!(
            parse_iso_date("2012-w04-5").unwrap(),
            (
                "",
                DateTimeSpec::Absolute(Absolute {
                    year: Some(2012),
                    week: Some(4),
                    weekday: Some(5),
                    ..Default::default()
                })
            )
        );
    }
}

#[cfg(test)]
mod test_parse {

    use super::*;

    #[test]
    fn dashed_date() {
        assert_eq!(
            parse("3-2-5").unwrap(),
            (
                "",
                DateTimeSpec::Absolute(Absolute {
                    year: Some(3),
                    month: Some(2),
                    day: Some(5),
                    ..Default::default()
                }),
            )
        );
    }

    #[test]
    fn test_slashed_date() {
        assert_eq!(
            parse("2/5/3").unwrap(),
            (
                "",
                DateTimeSpec::Absolute(Absolute {
                    year: Some(3),
                    month: Some(2),
                    day: Some(5),
                    ..Default::default()
                }),
            )
        );
    }

    #[test]
    fn test_day_of_month() {
        assert_eq!(
            parse("14").unwrap(),
            (
                "",
                DateTimeSpec::Absolute(Absolute {
                    day: Some(14),
                    ..Default::default()
                }),
            )
        );
    }

    #[test]
    fn test_month_slash_day() {
        assert_eq!(
            parse("2/5").unwrap(),
            (
                "",
                DateTimeSpec::Absolute(Absolute {
                    month: Some(2),
                    day: Some(5),
                    ..Default::default()
                }),
            )
        );
    }

    #[test]
    fn test_weekday() {
        assert_eq!(
            parse("Fri").unwrap(),
            (
                "",
                DateTimeSpec::Absolute(Absolute {
                    weekday: Some(5),
                    ..Default::default()
                }),
            )
        );
    }

    #[test]
    fn test_month_day() {
        assert_eq!(
            parse("sep 15").unwrap(),
            (
                "",
                DateTimeSpec::Absolute(Absolute {
                    month: Some(9),
                    day: Some(15),
                    ..Default::default()
                }),
            )
        );
        assert_eq!(
            parse("feb 15").unwrap(),
            (
                "",
                DateTimeSpec::Absolute(Absolute {
                    month: Some(2),
                    day: Some(15),
                    ..Default::default()
                }),
            )
        );
    }

    #[test]
    fn test_month_day_year() {
        assert_eq!(
            parse("sep 12 9").unwrap(),
            (
                "",
                DateTimeSpec::Absolute(Absolute {
                    year: Some(9),
                    month: Some(9),
                    day: Some(12),
                    ..Default::default()
                }),
            )
        );
    }

    #[test]
    fn test_hour_minute() {
        assert_eq!(
            parse("12:45").unwrap(),
            (
                "",
                DateTimeSpec::Absolute(Absolute {
                    minute: Some(45),
                    hour: Some(12),
                    ..Default::default()
                }),
            )
        );
    }

    #[test]
    fn test_day_month_hour_minute() {
        assert_eq!(
            parse("22 sept 0:34").unwrap(),
            (
                "",
                DateTimeSpec::Absolute(Absolute {
                    day: Some(22),
                    month: Some(9),
                    minute: Some(34),
                    hour: Some(0),
                    ..Default::default()
                }),
            )
        );
    }

    #[test]
    fn test_week_number() {
        assert_eq!(
            parse("w4").unwrap(),
            (
                "",
                DateTimeSpec::Absolute(Absolute {
                    week: Some(4),
                    ..Default::default()
                }),
            )
        );
    }

    #[test]
    fn test_iso_week_date() {
        assert_eq!(
            parse("2012 w4 fri").unwrap(),
            (
                "",
                DateTimeSpec::Absolute(Absolute {
                    year: Some(2012),
                    week: Some(4),
                    weekday: Some(5),
                    ..Default::default()
                }),
            )
        );
        assert_eq!(
            parse("2012-w04-5").unwrap(),
            (
                "",
                DateTimeSpec::Absolute(Absolute {
                    year: Some(2012),
                    week: Some(4),
                    weekday: Some(5),
                    ..Default::default()
                })
            ),
        );
    }

    #[test]
    fn test_empty() {
        assert!(matches!(parse(""), Err(nom::Err::Error(_))));
    }

    #[test]
    fn test_plus() {
        assert!(matches!(parse("+"), Err(nom::Err::Error(_))));
    }

    #[test]
    fn test_plus_zero() {
        assert_eq!(
            parse("+0").unwrap(),
            (
                "",
                DateTimeSpec::NowRelativeFuture(Relative {
                    days: Some(0),
                    ..Default::default()
                }),
            )
        );
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            parse(".").unwrap(),
            (
                "",
                DateTimeSpec::NowRelativeFuture(Relative {
                    ..Default::default()
                }),
            )
        )
    }

    #[test]
    fn test_plus_hours() {
        assert_eq!(
            parse("+3h").unwrap(),
            (
                "",
                DateTimeSpec::NowRelativeFuture(Relative {
                    hours: Some(3),
                    ..Default::default()
                }),
            )
        )
    }

    #[test]
    fn test_plus_days() {
        assert_eq!(
            parse("+4d").unwrap(),
            (
                "",
                DateTimeSpec::NowRelativeFuture(Relative {
                    days: Some(4),
                    ..Default::default()
                }),
            )
        )
    }

    #[test]
    fn test_plus_weeks() {
        assert_eq!(
            parse("+2w").unwrap(),
            (
                "",
                DateTimeSpec::NowRelativeFuture(Relative {
                    weeks: Some(2),
                    ..Default::default()
                }),
            )
        );
    }

    #[test]
    fn test_plus_months() {
        assert_eq!(
            parse("+3m").unwrap(),
            (
                "",
                DateTimeSpec::NowRelativeFuture(Relative {
                    months: Some(3),
                    ..Default::default()
                }),
            )
        );
    }

    #[test]
    fn test_plus_years() {
        assert_eq!(
            parse("+3y").unwrap(),
            (
                "",
                DateTimeSpec::NowRelativeFuture(Relative {
                    years: Some(3),
                    ..Default::default()
                }),
            )
        );
    }

    #[test]
    fn test_default_plus_days() {
        assert_eq!(
            parse("++5").unwrap(),
            (
                "",
                DateTimeSpec::DefaultRelativeFuture(Relative {
                    days: Some(5),
                    ..Default::default()
                }),
            )
        );
        assert_eq!(
            parse("--5").unwrap(),
            (
                "",
                DateTimeSpec::DefaultRelativePast(Relative {
                    days: Some(5),
                    ..Default::default()
                }),
            )
        );
    }

    #[test]
    fn test_relative_weekdays() {
        assert_eq!(
            parse("+2tue").unwrap(),
            (
                "",
                DateTimeSpec::NowRelativeFuture(Relative {
                    weekdays: Some(2),
                    weeks: Some(2),
                    ..Default::default()
                }),
            )
        );
        assert_eq!(
            parse("-wed").unwrap(),
            (
                "",
                DateTimeSpec::NowRelativePast(Relative {
                    weekdays: Some(3),
                    weeks: None,
                    ..Default::default()
                }),
            )
        );
    }

    #[test]
    fn test_time_range() {
        assert_eq!(
            parse("11am-1:15pm").unwrap(),
            (
                "",
                DateTimeSpec::TimeRangeAbsoluteStartAbsoluteEnd(
                    AbsoluteTime {
                        meridiem: Some(Meridiem::AM),
                        hour: Some(11),
                        minute: None
                    },
                    AbsoluteTime {
                        meridiem: Some(Meridiem::PM),
                        minute: Some(15),
                        hour: Some(1)
                    }
                ),
            )
        );
        assert_eq!(
            parse("11am--1:15pm").unwrap(),
            (
                "",
                DateTimeSpec::TimeRangeAbsoluteStartAbsoluteEnd(
                    AbsoluteTime {
                        meridiem: Some(Meridiem::AM),
                        hour: Some(11),
                        minute: None
                    },
                    AbsoluteTime {
                        meridiem: Some(Meridiem::PM),
                        minute: Some(15),
                        hour: Some(1)
                    }
                ),
            )
        );
    }

    #[test]
    fn test_time_duration() {
        assert_eq!(
            parse("11am+2:15").unwrap(),
            (
                "",
                DateTimeSpec::TimeRangeAbsoluteStartRelativeEnd(
                    AbsoluteTime {
                        meridiem: Some(Meridiem::AM),
                        hour: Some(11),
                        minute: None
                    },
                    RelativeTime {
                        hours: Some(2),
                        minutes: Some(15),
                    }
                ),
            )
        );
    }
}

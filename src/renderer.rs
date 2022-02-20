use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Weekday};

use crate::error::{OrgModeDateTimeError, Result};
use crate::parser::{Absolute, AbsoluteTime, DateTimeSpec, Meridiem, Relative, RelativeTime};

#[derive(Debug, PartialEq, Eq)]
pub enum RenderedSpec {
    Date(NaiveDate),
    DateTime(NaiveDateTime),
    Week(u32),
    TimeRange(NaiveTime, NaiveTime),
}

fn guess_abbreviated_year(now: NaiveDateTime, year: u32) -> Result<u32> {
    let result = if year < 10 {
        u32::try_from(now.year())? / 1000 * 1000 + year
    } else {
        year
    };
    Ok(result)
}

fn weekday_from_u32(weekday: u32) -> Weekday {
    match weekday {
        1 => Weekday::Mon,
        2 => Weekday::Tue,
        3 => Weekday::Wed,
        4 => Weekday::Thu,
        5 => Weekday::Fri,
        6 => Weekday::Sat,
        7 => Weekday::Sun,
        _ => panic!("weekday out of range: {}", weekday),
    }
}

fn apply_meridiem(hour: u32, meridiem: Option<Meridiem>) -> u32 {
    match meridiem {
        None | Some(Meridiem::AM) => hour,
        Some(Meridiem::PM) => hour + 12,
    }
}

fn render_absolute(absolute: Absolute, baseline: NaiveDateTime) -> Result<RenderedSpec> {
    match absolute {
        Absolute {
            year: Some(year),
            month: Some(month),
            day: Some(day),
            week: None,
            weekday: None,
            hour: None,
            minute: None,
            meridiem: None,
            ..
        } => {
            let date = NaiveDate::from_ymd(
                guess_abbreviated_year(baseline, year)?.try_into()?,
                month,
                day,
            );
            Ok(RenderedSpec::Date(date))
        }
        Absolute {
            year: None,
            month: None,
            day: Some(day),
            week: None,
            weekday: None,
            hour: None,
            minute: None,
            meridiem: None,
        } => {
            let mut date = NaiveDate::from_ymd(baseline.year(), baseline.month(), day);
            if date.and_hms(baseline.hour(), baseline.minute(), baseline.second()) < baseline {
                date = NaiveDate::from_ymd(baseline.year(), baseline.month() + 1, day);
            }
            Ok(RenderedSpec::Date(date))
        }
        Absolute {
            year: None,
            month: Some(month),
            day: Some(day),
            week: None,
            weekday: None,
            hour: None,
            minute: None,
            meridiem: None,
        } => {
            let mut date = NaiveDate::from_ymd(baseline.year(), month, day);
            if date.and_hms(baseline.hour(), baseline.minute(), baseline.second()) < baseline {
                date = NaiveDate::from_ymd(baseline.year() + 1, month, day);
            }
            Ok(RenderedSpec::Date(date))
        }
        Absolute {
            year: None,
            month: None,
            day: None,
            week: None,
            weekday: None,
            hour: Some(hour),
            minute: Some(minute),
            meridiem: None,
        } => {
            let mut datetime =
                NaiveDate::from_ymd(baseline.year(), baseline.month(), baseline.day())
                    .and_hms(hour, minute, 0);
            if datetime < baseline {
                datetime =
                    NaiveDate::from_ymd(datetime.year(), datetime.month(), datetime.day() + 1)
                        .and_hms(datetime.hour(), datetime.minute(), datetime.second());
            }
            Ok(RenderedSpec::DateTime(datetime))
        }
        Absolute {
            year: None,
            month: None,
            day: None,
            week: None,
            weekday: Some(weekday),
            hour: None,
            minute: None,
            meridiem: None,
        } => {
            let mut date = baseline.date();
            while date.weekday().number_from_monday() != weekday {
                date = date.succ();
            }
            Ok(RenderedSpec::Date(date))
        }
        Absolute {
            year: None,
            month: Some(month),
            day: Some(day),
            week: None,
            weekday: None,
            hour: Some(hour),
            minute: Some(minute),
            meridiem: None,
        } => {
            let mut datetime =
                NaiveDate::from_ymd(baseline.year(), month, day).and_hms(hour, minute, 0);
            if datetime < baseline {
                datetime =
                    NaiveDate::from_ymd(datetime.year() + 1, datetime.month(), datetime.day())
                        .and_hms(datetime.hour(), datetime.minute(), datetime.second());
            }
            Ok(RenderedSpec::DateTime(datetime))
        }
        Absolute {
            year: None,
            month: None,
            day: None,
            week: Some(week),
            weekday: None,
            hour: None,
            minute: None,
            meridiem: None,
        } => Ok(RenderedSpec::Week(week)),
        Absolute {
            year: Some(year),
            month: None,
            day: None,
            week: Some(week),
            weekday: Some(weekday),
            hour: None,
            minute: None,
            meridiem: None,
        } => {
            let date = NaiveDate::from_isoywd(year.try_into()?, week, weekday_from_u32(weekday));
            Ok(RenderedSpec::Date(date))
        }
        _ => unreachable!(),
    }
}

fn render_relative_future(relative: Relative, baseline: NaiveDateTime) -> Result<RenderedSpec> {
    match relative {
        // today
        Relative {
            hours: None,
            days: None,
            weeks: None,
            weekdays: None,
            months: None,
            years: None,
        } => Ok(RenderedSpec::Date(baseline.date())),
        // hours
        Relative {
            hours: Some(hours),
            days: None,
            weeks: None,
            weekdays: None,
            months: None,
            years: None,
        } => {
            let date = baseline + Duration::hours(hours.into());
            Ok(RenderedSpec::DateTime(date))
        }
        // days
        Relative {
            hours: None,
            days: Some(days),
            weeks: None,
            weekdays: None,
            months: None,
            years: None,
        } => {
            let date = baseline.date() + Duration::days(days.into());
            Ok(RenderedSpec::Date(date))
        }
        // weeks
        Relative {
            hours: None,
            days: None,
            weeks: Some(weeks),
            weekdays: None,
            months: None,
            years: None,
        } => {
            let date = baseline.date() + Duration::weeks(weeks.into());
            Ok(RenderedSpec::Date(date))
        }
        // weekdays
        Relative {
            days: None,
            hours: None,
            weeks,
            months: None,
            years: None,
            weekdays: Some(weekdays),
        } => {
            let mut date = baseline.date();
            while date.weekday().number_from_monday() != weekdays {
                date = date.succ();
            }
            let nweeks = weeks.unwrap_or(1);
            date += Duration::weeks(nweeks.into());
            Ok(RenderedSpec::Date(date))
        }
        // months
        Relative {
            hours: None,
            days: None,
            weeks: None,
            weekdays: None,
            months: Some(mut months),
            years: None,
        } => {
            months %= 12;
            let calculated_month0 = baseline.month0() + months;
            let (year, month) = if calculated_month0 > 11 {
                (baseline.year() + 1, calculated_month0 % 12 + 1)
            } else {
                (baseline.year(), calculated_month0 + 1)
            };
            let date = NaiveDate::from_ymd_opt(year, month, baseline.day()).ok_or(
                OrgModeDateTimeError::UnrepresentableFutureRelativeDate(relative),
            )?;
            Ok(RenderedSpec::Date(date))
        }
        // years
        Relative {
            hours: None,
            days: None,
            weeks: None,
            weekdays: None,
            months: None,
            years: Some(years),
        } => {
            let date = NaiveDate::from_ymd(
                baseline.year() + i32::try_from(years)?,
                baseline.month(),
                baseline.day(),
            );
            Ok(RenderedSpec::Date(date))
        }
        _ => unreachable!(),
    }
}

fn render_relative_past(relative: Relative, baseline: NaiveDateTime) -> Result<RenderedSpec> {
    match relative {
        // today
        Relative {
            hours: None,
            days: None,
            weeks: None,
            weekdays: None,
            months: None,
            years: None,
        } => Ok(RenderedSpec::Date(baseline.date())),
        // hours
        Relative {
            hours: Some(hours),
            days: None,
            weeks: None,
            weekdays: None,
            months: None,
            years: None,
        } => {
            let date = baseline - Duration::hours(hours.into());
            Ok(RenderedSpec::DateTime(date))
        }
        // days
        Relative {
            hours: None,
            days: Some(days),
            weeks: None,
            weekdays: None,
            months: None,
            years: None,
        } => {
            let date = baseline.date() - Duration::days(days.into());
            Ok(RenderedSpec::Date(date))
        }
        // weeks
        Relative {
            hours: None,
            days: None,
            weeks: Some(weeks),
            weekdays: None,
            months: None,
            years: None,
        } => {
            let date = baseline.date() - Duration::weeks(weeks.into());
            Ok(RenderedSpec::Date(date))
        }
        // weekdays
        Relative {
            days: None,
            hours: None,
            weeks,
            weekdays: Some(weekdays),
            months: None,
            years: None,
        } => {
            let weekday: Weekday = weekday_from_u32(weekdays);
            let mut date = baseline.date();
            while date.weekday() != weekday {
                date = date.pred();
            }
            let nweeks = weeks.unwrap_or(1) - 1;
            date -= Duration::weeks(nweeks.into());
            Ok(RenderedSpec::Date(date))
        }
        // months
        Relative {
            hours: None,
            days: None,
            weeks: None,
            weekdays: None,
            months: Some(mut months),
            years: None,
        } => {
            months %= 12;
            let calculated_month0 = i32::try_from(baseline.month0())? - i32::try_from(months)?;
            let (year, month) = if calculated_month0 < 0 {
                (baseline.year() - 1, -calculated_month0 % 12)
            } else {
                (baseline.year(), calculated_month0 + 1)
            };
            let date = NaiveDate::from_ymd_opt(year, u32::try_from(month)?, baseline.day()).ok_or(
                OrgModeDateTimeError::UnrepresentablePastRelativeDate(relative),
            )?;
            Ok(RenderedSpec::Date(date))
        }
        // years
        Relative {
            hours: None,
            days: None,
            weeks: None,
            weekdays: None,
            months: None,
            years: Some(years),
        } => {
            let date = NaiveDate::from_ymd(
                baseline.year() - i32::try_from(years)?,
                baseline.month(),
                baseline.day(),
            );
            Ok(RenderedSpec::Date(date))
        }
        _ => unreachable!(),
    }
}

fn render_time_range_absolute_start_absolute_end(
    start: AbsoluteTime,
    end: AbsoluteTime,
) -> Result<RenderedSpec> {
    match (start, end) {
        (
            AbsoluteTime {
                hour: Some(start_hour),
                minute: start_minute,
                meridiem: start_meridiem,
            },
            AbsoluteTime {
                hour: Some(end_hour),
                minute: end_minute,
                meridiem: end_meridiem,
            },
        ) => {
            let start_time = NaiveTime::from_hms(
                apply_meridiem(start_hour, start_meridiem),
                start_minute.unwrap_or(0),
                0,
            );
            let end_time = NaiveTime::from_hms(
                apply_meridiem(end_hour, end_meridiem),
                end_minute.unwrap_or(0),
                0,
            );
            Ok(RenderedSpec::TimeRange(start_time, end_time))
        }
        _ => unreachable!(),
    }
}

fn render_time_range_absolute_start_relative_end(
    start: AbsoluteTime,
    end: RelativeTime,
) -> Result<RenderedSpec> {
    match (start, end) {
        (
            AbsoluteTime {
                hour: Some(start_hour),
                minute: start_minute,
                meridiem: start_meridiem,
            },
            RelativeTime {
                hours: Some(end_hours),
                minutes: end_minutes,
            },
        ) => {
            let start_time = NaiveTime::from_hms(
                apply_meridiem(start_hour, start_meridiem),
                start_minute.unwrap_or(0),
                0,
            );
            let end_time = start_time
                + Duration::hours(end_hours.into())
                + Duration::minutes(end_minutes.unwrap_or(0).into());
            Ok(RenderedSpec::TimeRange(start_time, end_time))
        }
        _ => unreachable!(),
    }
}

pub fn render(
    default: NaiveDateTime,
    now: NaiveDateTime,
    spec: DateTimeSpec,
) -> Result<RenderedSpec> {
    match spec {
        DateTimeSpec::Absolute(absolute) => render_absolute(absolute, now),
        DateTimeSpec::NowRelativeFuture(relative) => render_relative_future(relative, now),
        DateTimeSpec::NowRelativePast(relative) => render_relative_past(relative, now),
        DateTimeSpec::DefaultRelativeFuture(relative) => render_relative_future(relative, default),
        DateTimeSpec::DefaultRelativePast(relative) => render_relative_past(relative, default),
        DateTimeSpec::TimeRangeAbsoluteStartAbsoluteEnd(start, end) => {
            render_time_range_absolute_start_absolute_end(start, end)
        }
        DateTimeSpec::TimeRangeAbsoluteStartRelativeEnd(start, end) => {
            render_time_range_absolute_start_relative_end(start, end)
        }
    }
}

import pytest
from datetime import date, datetime
from renderer import render, render_range


def absolute(d):
    return dict(type='absolute', base='default', **d)

def future(d):
    return dict(type='future', base='default', **d)

def past(d):
    return dict(type='past', base='default', **d)


def test_absolute_year_month_day():
    base = date(year=2006, month=6, day=13)
    input = absolute({'year': 3, 'month': 2, 'day': 5})
    assert render(input, base) == date(year=2003, month=2, day=5)

def test_absolute_day_of_month_before_base():
    base = date(year=2006, month=6, day=13)
    input = absolute({'day': 12})
    assert render(input, base) == date(year=2006, month=7, day=12)

def test_absolute_day_of_month_after_base():
    base = date(year=2006, month=6, day=13)
    input = absolute({'day': 14})
    assert render(input, base) == date(year=2006, month=6, day=14)

def test_absolute_month_day():
    base = date(year=2006, month=6, day=13)
    input = absolute({'month': 2, 'day': 5})
    assert render(input, base) == date(year=2007, month=2, day=5)

def test_absolute_weekday():
    base = date(year=2006, month=6, day=13)
    input = absolute({'weekday': 5})
    assert render(input, base) == date(year=2006, month=6, day=16)

def test_absolute_month_day_before_base():
    base = date(year=2006, month=6, day=13)
    input = absolute({'month': 2, 'day': 15})
    assert render(input, base) == date(year=2007, month=2, day=15)

def test_absolute_month_day_after_base():
    base = date(year=2006, month=6, day=13)
    input = absolute({'month': 9, 'day': 15})
    assert render(input, base) == date(year=2006, month=9, day=15)

def test_absolute_year_month_day():
    base = date(year=2006, month=6, day=13)
    input = absolute({'year': 9, 'month': 9, 'day': 12})
    assert render(input, base) == date(year=2009, month=9, day=12)

def test_absolute_hour_minute():
    base = date(year=2006, month=6, day=13)
    input = absolute({'start': {'hour': 12, 'minute': 45}})
    assert render(input, base) == datetime(year=2006, month=6, day=13, hour=12, minute=45)

def test_absolute_month_day_hour_minute():
    base = date(year=2006, month=6, day=13)
    input = absolute({'month': 9, 'day': 22, 'start': {'hour': 0, 'minute': 34}})
    assert render(input, base) == datetime(year=2006, month=9, day=22, hour=0, minute=34)

def test_absolute_week():
    base = date(year=2006, month=6, day=13)
    input = absolute({'week': 4})
    assert render(input, base) == date(year=2006, month=1, day=23)

def test_absolute_year_week_weekday():
    base = date(year=2006, month=6, day=13)
    input = absolute({'year': 2012, 'week': 4, 'weekday': 5})
    assert render(input, base) == date(year=2012, month=1, day=27)

def test_absolute_time_range():
    base = date(year=2006, month=6, day=13)
    input = absolute({'start': {'hour': 11, 'meridiem': 'am'}, 'end': {'hour': 1, 'meridiem': 'pm', 'minute': 15}})
    assert render_range(input, base) == (datetime(year=2006, month=6, day=13, hour=11, minute=0), datetime(year=2006, month=6, day=13, hour=13, minute=15))

def test_absolute_time_duration():
    base = date(year=2006, month=6, day=13)
    input = absolute({'start': {'hour': 11, 'meridiem': 'am'}, 'duration': {'hours': 2, 'minutes': 15}})
    assert render_range(input, base) == (datetime(year=2006, month=6, day=13, hour=11, minute=0), datetime(year=2006, month=6, day=13, hour=13, minute=15))


def test_future_empty():
    base = date(year=2006, month=6, day=13)
    input = future({})
    assert render(input, base) == date(year=2006, month=6, day=13)

def test_future_hours():
    base = date(year=2006, month=6, day=13)
    input = future({'hours': 3})
    assert render(input, base) == datetime(year=2006, month=6, day=13, hour=3)

def test_future_days():
    base = date(year=2006, month=6, day=13)

    zero = future({'days': 0})
    assert render(zero, base) == date(year=2006, month=6, day=13)

    four = future({'days': 4})
    assert render(four, base) == date(year=2006, month=6, day=17)

    five = future({'days': 5})
    assert render(five, base) == date(year=2006, month=6, day=18)

def test_future_weeks():
    base = date(year=2006, month=6, day=13)
    input = future({'weeks': 2})
    assert render(input, base) == date(year=2006, month=6, day=27)

def test_future_weeks_weekday():
    base = date(year=2006, month=6, day=13)
    input = future({'weeks': 2, 'weekday': 2})
    assert render(input, base) == date(year=2006, month=6, day=27)

def test_future_months():
    base = date(year=2006, month=6, day=13)
    input = future({'months': 3})
    assert render(input, base) == date(year=2006, month=9, day=13)

def test_future_years():
    base = date(year=2006, month=6, day=13)
    input = future({'years': 3})
    assert render(input, base) == date(year=2009, month=6, day=13)


def test_past_hours():
    base = date(year=2006, month=6, day=13)
    input = past({'hours': 3})
    assert render(input, base) == datetime(2006, 6, 12, 21)

def test_past_days():
    base = date(year=2006, month=6, day=13)
    input = past({'days': 5})
    assert render(input, base) == date(year=2006, month=6, day=8)

def test_past_weeks():
    base = date(year=2006, month=6, day=13)
    input = past({'weeks': 1})
    assert render(input, base) == date(year=2006, month=6, day=6)

def test_past_weekday():
    base = date(year=2006, month=6, day=13)
    input = past({'weekday': 3})
    assert render(input, base) == date(year=2006, month=6, day=7)

def test_past_months():
    base = date(year=2006, month=6, day=13)
    input = past({'months': 3})
    assert render(input, base) == date(year=2006, month=3, day=13)

def test_past_years():
    base = date(year=2006, month=6, day=13)
    input = past({'years': 3})
    assert render(input, base) == date(year=2003, month=6, day=13)

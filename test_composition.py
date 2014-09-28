from parser   import parse
from renderer import render, render_range
from datetime import date, datetime


def test_dashed_date():
    base = date(year=2006, month=6, day=13)
    dt,_ = parse('3-2-5')
    assert render(dt, base) == date(year=2003, month=2, day=5)

def test_slashed_date():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('2/5/3')
    assert render(dt, base) == date(year=2003, month=5, day=2)

def test_day_of_month():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('14')
    assert render(dt, base) == date(year=2006, month=6, day=14)

def test_day_month():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('2/5')
    assert render(dt, base) == date(year=2007, month=5, day=2)

def test_weekday():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('Fri')
    assert render(dt, base) == date(year=2006, month=6, day=16)

def test_month_day():
    base = date(year=2006, month=6, day=13)

    dt, _ = parse('sep 15')
    assert render(dt, base) == date(year=2006, month=9, day=15)

    dt, _ = parse('feb 15')
    assert render(dt, base) == date(year=2007, month=2, day=15)

def test_month_day_year():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('sep 12 9')
    assert render(dt, base) == date(year=2009, month=9, day=12)

def test_hour_minute():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('12:45')
    assert render(dt, base) == datetime(2006, 6, 13, 12, 45)

def test_day_month_hour_minute():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('22 sept 0:34')
    assert render(dt, base) == datetime(2006, 9, 22, 0, 34)

def test_week_number():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('w4')
    assert render(dt, base) == date(year=2006, month=1, day=23)

def test_iso_date():
    base = date(year=2006, month=6, day=13)

    dt, _ = parse('2012 w4 fri')
    assert render(dt, base) == date(year=2012, month=1, day=27)

    dt, _ = parse('2012-w04-5')
    assert render(dt, base) == date(year=2012, month=1, day=27)

def test_plus_zero():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('+0')
    assert render(dt, base) == base

def test_today():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('.')
    assert render(dt, base) == base

def test_plus_hours():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('+3h')
    assert render(dt, base) == datetime(2006, 6, 13, 3)

def test_plus_days():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('+4d')
    assert render(dt, base) == date(year=2006, month=6, day=17)

def test_default_plus_days():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('++5')
    assert render(dt, base) == date(year=2006, month=6, day=18)

def test_default_minus_days():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('--5')
    assert render(dt, base) == date(year=2006, month=6, day=8)

def test_plus_weeks():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('+2w')
    assert render(dt, base) == date(year=2006, month=6, day=27)

def test_plus_months():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('+3m')
    assert render(dt, base) == date(year=2006, month=9, day=13)

def test_plus_years():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('+3y')
    assert render(dt, base) == date(year=2009, month=6, day=13)

def test_plus_weekdays():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('+2tue')
    assert render(dt, base) == date(year=2006, month=6, day=27)

def test_minus_weekdays():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('-wed')
    assert render(dt, base) == date(year=2006, month=6, day=7)

def test_time_range():
    base = date(year=2006, month=6, day=13)

    dt, _ = parse('11am-1:15pm')
    assert render_range(dt, base) == (datetime(2006, 6, 13, 11), datetime(2006, 6, 13, 13, 15))

    dt, _ = parse('11am--1:15pm')
    assert render_range(dt, base) == (datetime(2006, 6, 13, 11), datetime(2006, 6, 13, 13, 15))

def test_time_duration():
    base = date(year=2006, month=6, day=13)
    dt, _ = parse('11am+2:15')
    assert render_range(dt, base) == (datetime(2006, 6, 13, 11), datetime(2006, 6, 13, 13, 15))

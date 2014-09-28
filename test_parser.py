from parser import parse


def test_dashed_date():
    assert parse('3-2-5') == ({'day': 5, 'base': 'default', 'year': 3, 'type': 'absolute', 'month': 2}, '')

def test_slashed_date():
    assert parse('2/5/3') == ({'day': 2, 'base': 'default', 'year': 3, 'type': 'absolute', 'month': 5}, '')

def test_day_of_month():
    assert parse('14') == ({'day': 14, 'base': 'default', 'type': 'absolute'}, '')

def test_day_month():
    assert parse('2/5') == ({'day': 2, 'base': 'default', 'type': 'absolute', 'month': 5}, '')

def test_week_day():
    assert parse('Fri') == ({'base': 'default', 'type': 'absolute', 'scale': 'weekday', 'weekday': 5}, '')

def test_month_day():
    assert parse('sep 15') == ({'day': 15, 'base': 'default', 'type': 'absolute', 'month': 9}, '')
    assert parse('feb 15') == ({'day': 15, 'base': 'default', 'type': 'absolute', 'month': 2}, '')

def test_month_day_year():
    assert parse('sep 12 9') == ({'day': 12, 'base': 'default', 'year': 9, 'type': 'absolute', 'month': 9}, '')

def test_hour_minute():
    assert parse('12:45') == ({'base': 'default', 'type': 'absolute', 'start': {'minute': 45, 'hour': 12}}, '')

def test_day_month_hour_minute():
    assert parse('22 sept 0:34') == ({'day': 22, 'base': 'default', 'type': 'absolute', 'month': 9, 'start': {'minute': 34, 'hour': 0}}, '')

def test_week_number():
    assert parse('w4') == ({'base': 'default', 'type': 'absolute', 'week': 4}, '')

def test_iso_date():
    assert parse('2012 w4 fri') == ({'base': 'default', 'year': 2012, 'type': 'absolute', 'week': 4, 'weekday': 5, 'scale': 'weekday'}, '')
    assert parse('2012-w04-5') == ({'base': 'default', 'year': 2012, 'type': 'absolute', 'week': 4, 'weekday': 5}, '')

def test_empty():
    assert parse('') == ({}, '')

def test_plus():
    assert parse('+') == ({}, '+')

def test_plus_zero():
    assert parse('+0') == ({'base': 'current', 'type': 'future', 'days': 0}, '')

def test_today():
    assert parse('.') == ({'base': 'current', 'type': 'future'}, '')

def test_plus_hours():
    assert parse('+3h') == ({'base': 'current', 'type': 'future', 'hours': 3}, '')

def test_plus_days():
    assert parse('+4d') == ({'base': 'current', 'type': 'future', 'days': 4}, '')

def test_plus_weeks():
    assert parse('+2w') == ({'base': 'current', 'type': 'future', 'weeks': 2}, '')

def test_plus_months():
    assert parse('+3m') == ({'base': 'current', 'type': 'future', 'months': 3}, '')

def test_plus_years():
    assert parse('+3y') == ({'base': 'current', 'type': 'future', 'years': 3}, '')

def test_default_plus_days():
    assert parse('++5') == ({'base': 'default', 'type': 'future', 'days': 5}, '')
    assert parse('--5') == ({'base': 'default', 'type': 'past', 'days': 5}, '')

def test_relative_weekdays():
    assert parse('+2tue') == ({'base': 'current', 'type': 'future', 'weekday': 2, 'weeks': 2}, '')
    assert parse('-wed') == ({'base': 'current', 'type': 'past', 'weekday': 3, 'weeks': 1}, '')

    # 'wed' === '+0wed'
    # '+wed' === '+1wed'
    # '-wed' === '-1wed'


def test_time_range():
    assert parse('11am-1:15pm') == ({'base': 'default', 'type': 'absolute', 'end': {'meridiem': 'pm', 'minute': 15, 'hour': 1}, 'start': {'meridiem': 'am', 'hour': 11}}, '')
    assert parse('11am--1:15pm') == ({'base': 'default', 'type': 'absolute', 'end': {'meridiem': 'pm', 'minute': 15, 'hour': 1}, 'start': {'meridiem': 'am', 'hour': 11}}, '')


def test_time_duration():
    assert parse('11am+2:15') == ({'base': 'default', 'type': 'absolute', 'duration': {'minutes': 15, 'hours': 2}, 'start': {'meridiem': 'am', 'hour': 11}}, '')

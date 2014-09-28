from datetime import date, datetime
from renderer import render, render_range


# Test cases mostly come from Org-mode manual examples:
# http://orgmode.org/manual/The-date_002ftime-prompt.html


def test_absolute():
    base = date(year=2006, month=6, day=13)
    absolute = lambda d: dict(type='absolute', base='default', **d)

    a1 = absolute({'year': 3, 'month': 2, 'day': 5})
    assert render(a1, base) == date(year=2003, month=2, day=5)

    a2 = absolute({'year': 3, 'month': 2, 'day': 5})
    assert render(a2, base) == date(year=2003, month=2, day=5)

    a3 = absolute({'day': 14})
    assert render(a3, base) == date(year=2006, month=6, day=14)

    a4 = absolute({'day': 12})
    assert render(a4, base) == date(year=2006, month=7, day=12)

    a5 = absolute({'month': 2, 'day': 5})
    assert render(a5, base) == date(year=2007, month=2, day=5)

    a6 = absolute({'weekday': 5})
    assert render(a6, base) == date(year=2006, month=6, day=16)

    a7 = absolute({'month': 9, 'day': 15})
    assert render(a7, base) == date(year=2006, month=9, day=15)

    a8 = absolute({'month': 2, 'day': 15})
    assert render(a8, base) == date(year=2007, month=2, day=15)

    a9 = absolute({'year': 9, 'month': 9, 'day': 12})
    assert render(a9, base) == date(year=2009, month=9, day=12)

    a10 = absolute({'start': {'hour': 12, 'minute': 45}})
    assert render(a10, base) == datetime(year=2006, month=6, day=13, hour=12, minute=45)

    a11 = absolute({'month': 9, 'day': 22, 'start': {'hour': 0, 'minute': 34}})
    assert render(a11, base) == datetime(year=2006, month=9, day=22, hour=0, minute=34)

    a12 = absolute({'week': 4})
    assert render(a12, base) == date(year=2006, month=1, day=23)

    a13 = absolute({'year': 2012, 'week': 4, 'weekday': 5})
    assert render(a13, base) == date(year=2012, month=1, day=27)

    a14 = absolute({'year': 2012, 'week': 4, 'weekday': 5})
    assert render(a14, base) == date(year=2012, month=1, day=27)

    a15 = absolute({'start': {'hour': 11, 'meridiem': 'am'}, 'end': {'hour': 1, 'meridiem': 'pm', 'minute': 15}})
    assert render_range(a15, base) == (datetime(year=2006, month=6, day=13, hour=11, minute=0), datetime(year=2006, month=6, day=13, hour=13, minute=15))

    a16 = absolute({'start': {'hour': 11, 'meridiem': 'am'}, 'end': {'hour': 1, 'minute': 15, 'meridiem': 'pm'}})
    assert render_range(a16, base) == (datetime(year=2006, month=6, day=13, hour=11, minute=0), datetime(year=2006, month=6, day=13, hour=13, minute=15))

    a17 = absolute({'start': {'hour': 11, 'meridiem': 'am'}, 'duration': {'hours': 2, 'minutes': 15}})
    assert render_range(a17, base) == (datetime(year=2006, month=6, day=13, hour=11, minute=0), datetime(year=2006, month=6, day=13, hour=13, minute=15))


def test_future():
    base = date(year=2006, month=6, day=13)
    future = lambda d: dict(type='future', base='default', **d)

    r1 = future({'days': 0})
    assert render(r1, base) == date(year=2006, month=6, day=13)

    r2 = future({})
    assert render(r2, base) == date(year=2006, month=6, day=13)

    r3 = future({'days': 4})
    assert render(r3, base) == date(year=2006, month=6, day=17)

    r4 = future({'days': 4})
    assert render(r4, base) == date(year=2006, month=6, day=17)

    r5 = future({'weeks': 2})
    assert render(r5, base) == date(year=2006, month=6, day=27)

    r6 = future({'days': 5})
    assert render(r6, base) == date(year=2006, month=6, day=18)

    r7 = future({'weeks': 2, 'weekday': 2})
    assert render(r7, base) == date(year=2006, month=6, day=27)

    r8 = future({'hours': 3})
    assert render(r8, base) == datetime(year=2006, month=6, day=13, hour=3)

    r9 = future({'months': 3})
    assert render(r9, base) == date(year=2006, month=9, day=13)

    r10 = future({'years': 3})
    assert render(r10, base) == date(year=2009, month=6, day=13)


def test_past():
    base = date(year=2006, month=6, day=13)
    past = lambda d: dict(type='past', base='default', **d)

    r8 = past({'weekday': 3})
    assert render(r8, base) == date(year=2006, month=6, day=7)

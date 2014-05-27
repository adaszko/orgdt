from datetime import date
from dateutil.relativedelta import relativedelta
import calendar
import pytest
from dtrenderer import render


@pytest.mark.parametrize("base", [date(year=2000, month=1, day=1)])
def test_org_mode_docs_examples(base):
    assert render({'day': 5, 'base': 'current', 'year': 3, 'month': 2, 'type': 'absolute'}, base) == date(year=2003, month=2, day=5)
    assert render({'day': 2, 'base': 'current', 'year': 3, 'month': 5, 'type': 'absolute'}, base) == date(year=2003, month=5, day=2)
    assert render({'day': 14, 'base': 'current', 'type': 'absolute'}, base) == base + relativedelta(day=14)
    assert render({'day': 2, 'base': 'current', 'month': 5, 'type': 'absolute'}, base) == base + relativedelta(month=5, day=2)

    assert render({'base': 'current', 'type': 'absolute', 'weekday': 4}, base) == base + relativedelta(weekday=calendar.FRIDAY)
    assert render({'day': 15, 'base': 'current', 'month': 9, 'type': 'absolute'}, base) == base + relativedelta(month=9, day=15)
    assert render({'day': 15, 'base': 'current', 'month': 2, 'type': 'absolute'}, base) == base + relativedelta(day=15, month=2)
    assert render({'day': 12, 'base': 'current', 'year': 9, 'month': 9, 'type': 'absolute'}, base) == base + relativedelta(day=12, month=9, year=2009)
    assert render({'base': 'current', 'type': 'absolute', 'start': {'minute': 45, 'hour': 12}}, base) == base + relativedelta(hour=12, minute=45)
    assert render({'day': 22, 'base': 'current', 'month': 9, 'type': 'absolute', 'start': {'minute': 34, 'hour': 0}}, base) == base + relativedelta(month=9, day=22, hour=0, minute=34)
    assert render({'base': 'current', 'type': 'absolute', 'week': 4}, base) == date(year=base.year, month=1, day=1) + relativedelta(weeks=4, weekday=base.weekday())
    assert render({'base': 'current', 'year': 2012, 'type': 'absolute', 'week': 4, 'weekday': 4}, base) == date(year=2012, month=1, day=1) + relativedelta(weeks=4, weekday=calendar.FRIDAY)

    assert render({'base': 'current', 'type': 'future', 'days': 0, 'months': 0, 'years': 0}, base) == base
    assert render({'base': 'current', 'type': 'future', 'days': 0, 'months': 0, 'years': 0}, base) == base
    assert render({'base': 'current', 'type': 'future', 'days': 4, 'months': 0, 'years': 0}, base) == base + relativedelta(days=4)
    assert render({'base': 'current', 'type': 'future', 'weeks': 2, 'days': 0, 'months': 0, 'years': 0}, base) == base + relativedelta(weeks=2)
    assert render({'base': 'default', 'type': 'future', 'days': 5, 'months': 0, 'years': 0}, base) == base + relativedelta(days=5)
    assert render({'base': 'current', 'weeks': 2, 'type': 'future', 'weekday': 1}, base) == base + relativedelta(weeks=2, weekday=calendar.TUESDAY)
    assert render({'base': 'current', 'weeks': 1, 'type': 'past', 'weekday': 2, 'days': 0}, base) == base - relativedelta(weeks=1, weekday=calendar.WEDNESDAY)

    assert render({'end': {'meridiem': 'pm', 'minute': 15, 'hour': 1}, 'type': 'absolute', 'base': 'current', 'start': {'meridiem': 'am', 'hour': 11}}, base) == (base+relativedelta(hour=11), base+relativedelta(hour=13, minute=15))
    assert render({'end': {'meridiem': 'pm', 'minute': 15, 'hour': 1}, 'type': 'absolute', 'base': 'current', 'start': {'meridiem': 'am', 'hour': 11}}, base) == (base+relativedelta(hour=11), base+relativedelta(hour=13, minute=15))
    assert render({'base': 'current', 'start': {'meridiem': 'am', 'hour': 11}, 'type': 'absolute', 'duration': {'minutes': 15, 'hours': 2}}, base) == (base+relativedelta(hour=11), base+relativedelta(hour=11, hours=2, minutes=15))

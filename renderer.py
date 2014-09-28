#!/usr/bin/env python

from datetime import date
from datetime import datetime
from dateutil.relativedelta import relativedelta, weekday


class RenderException(Exception):
    pass

class BaseNotPresent(RenderException):
    pass

class UnknownBaseType(RenderException):
    pass

class TypeNotPresent(RenderException):
    pass

class UnknownDateTimeType(RenderException):
    pass

class UnknownMeridiemSpecifier(RenderException):
    pass

class UnknownRangeType(RenderException):
    pass

class UnrecognizedSpec(RenderException):
    pass


def _render_time(dtspec, day):
    if 'hour' in dtspec or 'minute' in dtspec:
        return datetime(year=day.year, month=day.month, day=day.day, hour=dtspec.get('hour', 0), minute=dtspec.get('minute', 0))

    return day


def _render_absolute(dtspec, base):
    if 'year' in dtspec and 'month' in dtspec and 'day' in dtspec:
        day = date(year=dtspec['year'], month=dtspec['month'], day=dtspec['day'])
        return _render_time(dtspec, day)

    if 'month' in dtspec and 'day' in dtspec:
        day = date(year=base.year, month=dtspec['month'], day=dtspec['day'])
        if day < base:
            day += relativedelta(years=1)
        return _render_time(dtspec, day)

    if 'day' in dtspec:
        day = date(year=base.year, month=base.month, day=dtspec['day'])
        if day < base:
            day += relativedelta(months=1)
        return _render_time(dtspec, day)

    if 'year' in dtspec and 'week' in dtspec and 'weekday' in dtspec:
        wday = weekday(dtspec['weekday'] - 1)
        day = date(year=dtspec['year'], month=1, day=1) + relativedelta(weekday=wday, weeks=dtspec['week']-1)
        return _render_time(dtspec, day)

    if 'weekday' in dtspec:
        wday = weekday(dtspec['weekday'] - 1)
        day = base + relativedelta(weekday=wday)
        return _render_time(dtspec, day)

    if 'week' in dtspec:
        wday = weekday(base.weekday())
        day = date(year=base.year, month=1, day=1) + relativedelta(day=4, weekday=weekday(0, -1), weeks=dtspec['week']-1)
        return _render_time(dtspec, day)

    return _render_time(dtspec, base)


def _make_relative_delta(dtspec, is_past):
    if 'years' in dtspec:
        return relativedelta(years=dtspec['years'])

    if 'months' in dtspec:
        return relativedelta(months=dtspec['months'])

    if 'weeks' in dtspec and 'weekday' in dtspec:
        n = -dtspec['weeks'] if is_past else dtspec['weeks'] + 1
        wday = weekday(dtspec['weekday'] - 1, n)
        return relativedelta(weekday=wday)

    if 'weeks' in dtspec:
        return relativedelta(weeks=dtspec['weeks'])

    if 'weekday' in dtspec:
        n = -1 if is_past else 1
        wday = weekday(dtspec['weekday'] - 1, n)
        return relativedelta(weekday=wday)

    if 'days' in dtspec:
        return relativedelta(days=dtspec['days'])

    if 'hours' in dtspec and 'minutes' in dtspec:
        return relativedelta(hours=dtspec['hours'], minutes=dtspec['minutes'])

    if 'hours' in dtspec:
        return relativedelta(hours=dtspec['hours'])

    if 'minutes' in dtspec:
        return relativedelta(minutes=dtspec['minutes'])

    return relativedelta()


def _normalize_year(year, base):
    if year >= 100:
        return year

    if year >= 10:
        return base.year / 100 * 100 + year

    return base.year / 10 * 10 + year


def _render_type(dtspec, base):
    without_type = dict((k, v) for k, v in dtspec.iteritems() if k != 'type')

    if dtspec['type'] == 'absolute':
        if 'year' in dtspec:
            without_type['year'] = _normalize_year(dtspec['year'], base)
        return _render_absolute(without_type, base)

    if dtspec['type'] in ('future', 'past'):
        return base + _make_relative_delta(without_type, dtspec['type'] == 'past')

    raise UnknownDateTimeType(dtspec['type'])


def _normalize_time_spec(timespec):
    """
    >>> _normalize_time_spec({'meridiem': 'am', 'hour': 0})
    {'hour': 24, 'minute': 0}
    """

    if 'meridiem' in timespec:
        if timespec['meridiem'] not in ('am', 'pm'):
            raise UnknownMeridiemSpecifier(timespec['meridiem'])
        if 'minute' not in timespec:
            timespec['minute'] = 0
        if timespec['meridiem'] == 'pm':
            return {'hour': timespec['hour'] + 12, 'minute': timespec['minute']}
        return timespec

    return timespec


def _render_point(dtspec, default, current):
    without_base = dict((k, v) for k, v in dtspec.iteritems() if k != 'base')

    if dtspec['base'] == 'default':
        return _render_type(without_base, default)

    if dtspec['base'] == 'current':
        return _render_type(without_base, current)

    raise UnknownBaseType()


def render(dtspec, default, current=None):
    if 'start' not in dtspec:
        return _render_point(dtspec, default, current)

    pointized = dict((k, v) for k, v in dtspec.iteritems() if k != 'start')
    normalized_start = _normalize_time_spec(dtspec['start'])
    start_dtspec = dict(hour=normalized_start['hour'], minute=normalized_start['minute'], **pointized)
    return _render_point(start_dtspec, default, current)


def render_range(dtspec, default, current=None):
    pointized = dict((k, v) for k, v in dtspec.iteritems() if k not in ('end', 'duration'))

    rendered_start = render(dtspec, default, current)

    if 'end' in dtspec:
        normalized_end = _normalize_time_spec(dtspec['end'])
        end_dtspec = dict(hour=normalized_end['hour'], minute=normalized_end['minute'], **pointized)
        rendered_end = _render_point(end_dtspec, default=rendered_start, current=None)
        return (rendered_start, rendered_end)

    if 'duration' in dtspec:
        duration_dtspec = {'base': 'default', 'type': 'future'}
        if 'hours' in dtspec['duration']:
            duration_dtspec['hours'] = dtspec['duration']['hours']
        if 'minutes' in dtspec['duration']:
            duration_dtspec['minutes'] = dtspec['duration']['minutes']
        rendered_duration = _render_point(duration_dtspec, default=rendered_start, current=None)
        return (rendered_start, rendered_duration)

    raise UnknownRangeType(dtspec)

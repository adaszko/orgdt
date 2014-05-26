#!/usr/bin/env python

from datetime import date
from dateutil.relativedelta import relativedelta


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


def make_relative_delta(dtspec):
    return relativedelta(years=dtspec.get('years', 0),
                         months=dtspec.get('months', 0),
                         days=dtspec.get('days', 0),
                         weeks=dtspec.get('weeks', 0),
                         weekday=dtspec.get('weekday'))


def _render_based(dtspec, base):
    if 'type' not in dtspec:
        raise TypeNotPresent(None)

    if dtspec['type'] == 'absolute':
        normalized_start = _normalize_time_spec(dtspec['start']) if dtspec.get('start') else {}

        if 'week' in dtspec:
            return date(year=dtspec.get('year', base.year),
                        month=1,
                        day=1) + relativedelta(weeks=dtspec['week'],
                                               weekday=dtspec.get('weekday', base.weekday()))

        if 'weekday' in dtspec:
            return base + relativedelta(weekday=dtspec['weekday'])

        return base + relativedelta(year=dtspec.get('year'),
                                    month=dtspec.get('month'),
                                    day=dtspec.get('day'),
                                    hour=normalized_start.get('hour'),
                                    minute=normalized_start.get('minute'))
    elif dtspec['type'] == 'past':
        fixed = dict(**dtspec)
        if 'weeks' in fixed:
            fixed['weeks'] -= 1
        return base - make_relative_delta(fixed)
    elif dtspec['type'] == 'future':
        return base + make_relative_delta(dtspec)
    else:
        raise UnknownDateTimeType(dtspec['type'])


def _render_start(dtspec, current, default):
    if not 'base' in dtspec:
        raise BaseNotPresent()

    if dtspec['base'] == 'current':
        return _render_based(dtspec, current)
    elif dtspec['base'] == 'default':
        base = default if default is not None else current
        return _render_based(dtspec, base)
    else:
        raise UnknownBaseType()


def _normalize_time_spec(timespec):

    """
    >>> _normalize_time_spec({'meridiem': 'am', 'hour': 0)
    {'hour': 24, 'minute': 0}
    """

    if 'meridiem' in timespec:
        if timespec['meridiem'] not in ('am', 'pm'):
            raise UnknownMeridiemSpecifier(timespec['meridiem'])
        if timespec['meridiem'] == 'pm':
            return {'hour': timespec['hour'] + 12, 'minute': timespec['minute']}
        return timespec

    return timespec


def render(dtspec, current, default=None):
    if 'start' in dtspec and 'end' in dtspec:
        start = _render_start(dtspec, current, default)
        normalized_end = _normalize_time_spec(dtspec['end'])
        end = start + relativedelta(hour=normalized_end['hour'], minute=normalized_end['minute'])
        return (start, end)

    if 'start' in dtspec and 'duration' in dtspec:
        start = _render_start(dtspec, current, default)
        duration = dtspec.get('duration', {})
        end = start + relativedelta(hours=duration.get('hours', 0), minutes=duration.get('minutes', 0))
        return (start, end)

    return _render_start(dtspec, current, default)

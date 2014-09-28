import re


class ParseError(Exception):
    pass


def _add_to(dict_, **fields):
    return dict(dict_.items(), **fields)


def _merge(lhs, rhs):
    return dict(lhs.items(), **rhs)


def _empty(parsee):
    return None, parsee


def _literal(string, parsee):
    if not parsee.startswith(string):
        raise ParseError(parsee)

    return string, parsee[len(string):]


def _regex(regex, parsee, flags=0):
    match = re.match(regex, parsee, flags)
    if match is None:
        raise ParseError(parsee)

    rest = parsee[match.end():]
    return match.group(), rest


def _regex_respect_case(regex, parsee):
    return _regex(regex, parsee)


def _regex_ignore_case(regex, parsee):
    return _regex(regex, parsee, re.IGNORECASE)


def _sequence(parsers, parsee):
    rest = parsee
    result = []
    for parser in parsers:
        token, rest = parser(rest)
        result.append(token)
    return result, rest


def _alternative(parsers, parsee):
    for parser in parsers:
        try:
            return parser(parsee)
        except ParseError:
            continue
    raise ParseError(parsee)


def _optional(parser, parsee):
    tokens, rest = _alternative([parser, _empty], parsee)

    if tokens is None:
        return None, rest

    return tokens, rest


def _today_symbol(parsee):
    return _literal('.', parsee)


def _dash(parsee):
    return _literal('-', parsee)


def _colon(parsee):
    return _literal(':', parsee)


def _plus(parsee):
    return _literal('+', parsee)


def _minus(parsee):
    return _literal('-', parsee)


def _slash(parsee):
    return _literal('/', parsee)


def _dot(parsee):
    return _literal('.', parsee)


def _hours_symbol(parsee):
    _, rest = _literal('h', parsee)
    return {'scale': 'hours'}, rest


def _days_symbol(parsee):
    _, rest = _literal('d', parsee)
    return {'scale': 'days'}, rest


def _number(parsee):
    token, rest = _regex_respect_case('\d+', parsee)

    try:
        n = int(token)
    except ValueError:
        raise ParseError(token)

    return [n, rest]


def _monday(parsee):
    _, rest = _regex_ignore_case('mon(d(ay?)?)?', parsee)
    return 1, rest


def _tuesday(parsee):
    _, rest = _regex_ignore_case('tue(s(d(a(y?)?)?)?)?', parsee)
    return 2, rest


def _wednesday(parsee):
    _, rest = _regex_ignore_case('wed(n(e(s(d(ay?)?)?)?)?)?', parsee)
    return 3, rest


def _thurdsay(parsee):
    _, rest = _regex_ignore_case('thu(r(s(d(ay?)?)?)?)?', parsee)
    return 4, rest


def _friday(parsee):
    _, rest = _regex_ignore_case('fri(d(ay?)?)?', parsee)
    return 5, rest


def _saturday(parsee):
    _, rest = _regex_ignore_case('sat(u(r(d(ay?)?)?)?)?', parsee)
    return 6, rest


def _sunday(parsee):
    _, rest = _regex_ignore_case('sun(d(ay?)?)?', parsee)
    return 7, rest


def _january(parsee):
    _, rest = _regex_ignore_case('jan(u(a(ry?)?)?)?', parsee)
    return 1, rest


def _february(parsee):
    _, rest = _regex_ignore_case('feb(r(u(a(ry?)?)?)?)?', parsee)
    return 2, rest


def _march(parsee):
    _, rest = _regex_ignore_case('mar(ch?)?', parsee)
    return 3, rest


def _april(parsee):
    _, rest = _regex_ignore_case('apr(il?)?', parsee)
    return 4, rest


def _may(parsee):
    _, rest = _regex_ignore_case('may', parsee)
    return 5, rest


def _june(parsee):
    _, rest = _regex_ignore_case('june?', parsee)
    return 6, rest


def _july(parsee):
    _, rest = _regex_ignore_case('july?', parsee)
    return 7, rest


def _august(parsee):
    _, rest = _regex_ignore_case('aug(u(st?)?)?', parsee)
    return 8, rest


def _september(parsee):
    _, rest = _regex_ignore_case('sep(t(e(m(b(er?)?)?)?)?)?', parsee)
    return 9, rest


def _october(parsee):
    _, rest = _regex_ignore_case('oct(o(b(er?)?)?)?', parsee)
    return 10, rest


def _november(parsee):
    _, rest = _regex_ignore_case('nov(e(m(b(er?)?)?)?)?', parsee)
    return 11, rest


def _december(parsee):
    _, rest = _regex_ignore_case('dec(e(m(b(er?)?)?)?)?', parsee)
    return 12, rest


def _spaces(parsee):
    token, rest = _regex_respect_case('\s*', parsee)
    return [token, rest]


def _day_name(parsee):
    token, rest = _alternative([_monday, _tuesday, _wednesday, _thurdsay, _friday, _saturday, _sunday], parsee)
    return {'scale': 'weekday', 'weekday': token}, rest


def _weeks_symbol(parsee):
    _, rest = _literal('w', parsee)
    return {'scale': 'weeks'}, rest


def _months_symbol(parsee):
    _, rest = _literal('m', parsee)
    return {'scale': 'months'}, rest


def _years_symbol(parsee):
    _, rest = _literal('y', parsee)
    return {'scale': 'years'}, rest


def _date_offset_scale(parsee):
    return _alternative([_hours_symbol, _days_symbol, _day_name, _weeks_symbol, _months_symbol, _years_symbol], parsee)


def _optional_date_offset_scale(parsee):
    return _optional(_date_offset_scale, parsee)


def _make_scale(number, tokens):
    scale = tokens['scale']
    if scale == 'hours':
        return {'hours': number}
    if scale == 'days':
        return {'days': number}
    if scale == 'weekday':
        return {'weeks': number, 'weekday': tokens['weekday']}
    if scale == 'weeks':
        return {'weeks': number}
    if scale == 'months':
        return {'months': number}
    if scale == 'years':
        return {'years': number}
    assert False, scale


def _number_optional_date_offset_scale(parsee):
    (number, optional_scale), rest = _sequence([_number, _optional_date_offset_scale], parsee)

    if optional_scale:
        return _make_scale(number, optional_scale), rest

    return dict(days=number), rest


def _optional_number(parsee):
    return _optional(_number, parsee)


def _optional_number_date_offset_scale(parsee):
    (number, scale), rest = _sequence([_optional_number, _date_offset_scale], parsee)
    return _make_scale(number if number is not None else 1, scale), rest


def _date_offset(parsee):
    return _alternative([_number_optional_date_offset_scale, _optional_number_date_offset_scale], parsee)


def _today_relative_future_date_time(parsee):
    (_, date_offset), rest = _sequence([_plus, _date_offset], parsee)
    return _add_to(date_offset, type='future'), rest


def _today_relative_past_date_time(parsee):
    (_, date_offset), rest = _sequence([_minus, _date_offset], parsee)
    return _add_to(date_offset, type='past'), rest


def _today_relative_date_time(parsee):
    tokens, rest = _alternative([_today_relative_future_date_time, _today_relative_past_date_time], parsee)
    return _add_to(tokens, base='current'), rest


def _default_date_plus(parsee):
    return _literal('++', parsee)


def _default_date_minus(parsee):
    return _literal('--', parsee)


def _default_date_future_date_time(parsee):
    (_, date_offset), rest = _sequence([_default_date_plus, _date_offset], parsee)
    return _add_to(date_offset, type='future'), rest


def _default_date_past_date_time(parsee):
    (_, date_offset), rest = _sequence([_default_date_minus, _date_offset], parsee)
    return _add_to(date_offset, type='past'), rest


def _default_date_relative_date_time(parsee):
    relative_date_time, rest = _alternative([_default_date_future_date_time, _default_date_past_date_time], parsee)
    return dict(base='default', **relative_date_time), rest


def _dashed_date(parsee):
    tokens, rest = _sequence([_number, _dash, _number, _dash, _number], parsee)
    year, _, month, _, day = tokens
    result = {'year': year, 'month': month, 'day': day}
    return result, rest


def _slash_number(parsee):
    return _sequence([_slash, _number], parsee)


def _optional_slash_number(parsee):
    return _optional(_slash_number, parsee)


def _slashed_date(parsee):
    (day, _, month, optional_slash_year), rest = _sequence([_number, _slash, _number, _optional_slash_number], parsee)

    if optional_slash_year:
        _, year = optional_slash_year
        return dict(day=day, month=month, year=year), rest

    return dict(day=day, month=month), rest


def _dotted_date(parsee):
    (day, _, month, _, year), rest = _sequence([_number, _dot, _number, _dot, _number], parsee)
    return [{'day': day, 'month': month, 'year': year}, rest]


def _month_name(parsee):
    return _alternative([_january, _february, _march, _april, _june, _july, _august, _september, _october, _november, _december], parsee)


def _spaces_number(parsee):
    return _sequence([_spaces, _number], parsee)


def _optional_spaces_number(parsee):
    return _optional(_spaces_number, parsee)


def _month_day(parsee):
    (month, _, day, optional_spaces_year), rest = _sequence([_month_name, _spaces, _number, _optional_spaces_number], parsee)

    if optional_spaces_year:
        _, year = optional_spaces_year
        return dict(month=month, day=day, year=year), rest

    return dict(month=month, day=day), rest


def _week_number(parsee):
    tokens, rest = _sequence([_weeks_symbol, _number], parsee)
    return [{'week': tokens[1]}, rest]


def _year_week_number(parsee):
    (year, _, week, _, day), rest = _sequence([_number, _spaces, _week_number, _spaces, _day_name], parsee)
    result = _merge(week, day)
    return _add_to(result, year=year), rest


def _dashed_year_week_number(parsee):
    (year, _, week, _, day), rest = _sequence([_number, _dash, _week_number, _dash, _number], parsee)
    return _add_to(week, year=year, weekday=day), rest


def _day_month(parsee):
    (day, _, month), rest = _sequence([_number, _spaces, _month_name], parsee)
    return dict(day=day, month=month), rest


def _date(parsee):
    return _alternative([_dashed_date, _slashed_date, _dotted_date, _month_day, _day_month, _week_number, _year_week_number, _dashed_year_week_number], parsee)


def _hour_colon_minute(parsee):
    tokens, rest = _sequence([_number, _colon, _number], parsee)

    result = {
        'hour':     tokens[0],
        'minute':   tokens[2],
    }

    return [result, rest]


def _ante_meridiem(parsee):
    _, rest = _regex_ignore_case('am', parsee)
    return ['am', rest]


def _post_meridiem(parsee):
    _, rest = _regex_ignore_case('pm', parsee)
    return ['pm', rest]


def _meridiem(parsee):
    return _alternative([_ante_meridiem, _post_meridiem], parsee)


def _optional_meridiem(parsee):
    return _optional(_meridiem, parsee)


def _hour_meridiem(parsee):
    tokens, rest = _sequence([_number, _meridiem], parsee)

    result = {
        'hour':         tokens[0],
        'meridiem':     tokens[1],
    }

    return [result, rest]


def _hour_colon_minute_optional_meridiem(parsee):
    (time, optional_meridiem), rest = _sequence([_hour_colon_minute, _optional_meridiem], parsee)

    if optional_meridiem:
        return _add_to(time, meridiem=optional_meridiem), rest

    return time, rest


def _time_optional_meridiem(parsee):
    return _alternative([_hour_colon_minute_optional_meridiem, _hour_meridiem], parsee)


def _colon_number(parsee):
    return _sequence([_colon, _number], parsee)


def _optional_colon_number(parsee):
    return _optional(_colon_number, parsee)


def _duration(parsee):
    (hours, optional_colon_number), rest = _sequence([_number, _optional_colon_number], parsee)

    if optional_colon_number:
        _, minutes = optional_colon_number
        return dict(hours=hours, minutes=minutes), rest

    return dict(hours=hours), rest


def _time_duration(parsee):
    tokens, rest = _sequence([_time_optional_meridiem, _plus, _duration], parsee)

    result = {
        'start':    tokens[0],
        'duration': tokens[2],
    }

    return [result, rest]


def _range_sep(parsee):
    return _regex_respect_case('--?', parsee)


def _range_sep_hour_minute_meridiem(parsee):
    return _sequence([_range_sep, _time_optional_meridiem], parsee)


def _optional_range_sep_hour_colon_minute_meridiem(parsee):
    optional_range_sep_time, rest = _optional(_range_sep_hour_minute_meridiem, parsee)

    if optional_range_sep_time:
        _, time = optional_range_sep_time
        return time, rest

    return None, rest


def _time_range(parsee):
    (start, optional_end), rest = _sequence([_time_optional_meridiem, _optional_range_sep_hour_colon_minute_meridiem], parsee)

    if optional_end:
        return dict(start=start, end=optional_end), rest

    return dict(start=start), rest


def _time(parsee):
    return _alternative([_time_duration, _time_range], parsee)


def _optional_time(parsee):
    return _optional(_time, parsee)


def _date_optional_time(parsee):
    (date, _, optional_time), rest = _sequence([_date, _spaces, _optional_time], parsee)

    if optional_time:
        return _merge(date, optional_time), rest

    return date, rest


def _optional_date(parsee):
    return _optional(_date, parsee)


def _time_optional_date(parsee):
    (time, _, maybe_date), rest = _sequence([_time, _spaces, _optional_date], parsee)

    if maybe_date:
        return _merge(time, maybe_date), rest

    return time, rest


def _day_of_month(parsee):
    day, rest = _number(parsee)
    return [{'day': day}, rest]


def _date_time(parsee):
    return _alternative([_day_name, _date_optional_time, _time_optional_date, _day_of_month], parsee)


def _absolute_date_time(parsee):
    tokens, rest = _date_time(parsee)
    result = dict(base='default', type='absolute', **tokens)
    return result, rest


def _just_today_symbol(parsee):
    _, rest = _today_symbol(parsee)
    return dict(base='current', type='future'), rest


def _date_time_string(parsee):
    return _alternative([_today_relative_date_time, _default_date_relative_date_time, _absolute_date_time, _just_today_symbol], parsee)


def parse(parsee):
    try:
        return _date_time_string(parsee)
    except ParseError:
        return {}, parsee

" References:
" [1] http://orgmode.org/manual/The-date_002ftime-prompt.html#The-date_002ftime-prompt

" {{{ Terminal symbols
" Returns: a pair [MATCHED-STRING, REST-OF-A:STRING]
" Throws an exception if a:string doesn't match a:regex
function! RegEx(regex, string)
    " ^ should work regardless of &magic's setting
    let regex = '^' . a:regex
    let match = matchstr(a:string, regex)

    " matchstr() returns an empty string when match wasn't found but also
    " when the match was an empty string.  We use match() to differentiate
    " between these two cases.
    if empty(match) && match(a:string, regex) < 0
	throw 'PARSING: RegEx: ' . a:regex . a:string
    endif

    return [match, strpart(a:string, strlen(match))]
endfunction

function! Empty(string)
    return [[], a:string]
endfunction

function! Spaces(string)
    return RegEx('\v\s*', a:string)
endfunction

function! Colon(string)
    return RegEx(':', a:string)
endfunction

function! Dash(string)
    return RegEx('-', a:string)
endfunction

function! RangeSep(string)
    return RegEx('\v--?', a:string)
endfunction

function! Slash(string)
    return RegEx('/', a:string)
endfunction

function! Plus(string)
    return RegEx('\V+', a:string)
endfunction

function! Minus(string)
    return RegEx('\V-', a:string)
endfunction

function! Dot(string)
    return RegEx('\V.', a:string)
endfunction

function! Number(string)
    let [token, rest] = RegEx('\v\d+', a:string)
    return [str2nr(token), rest]
endfunction

function! Sunday(string)
    let [_, rest] = RegEx('\c\vsun(d(ay?)?)?', a:string)
    return [0, rest]
endfunction

function! Monday(string)
    let [_, rest] = RegEx('\c\vmon(d(ay?)?)?', a:string)
    return [1, rest]
endfunction

function! Tuesday(string)
    let [_, rest] = RegEx('\c\vtue(s(d(a(y?)?)?)?)?', a:string)
    return [2, rest]
endfunction

function! Wednesday(string)
    let [_, rest] = RegEx('\c\vwed(n(e(s(d(ay?)?)?)?)?)?', a:string)
    return [3, rest]
endfunction

function! Thursday(string)
    let [_, rest] = RegEx('\c\vthu(r(s(d(ay?)?)?)?)?', a:string)
    return [4, rest]
endfunction

function! Friday(string)
    let [_, rest] = RegEx('\c\vfri(d(ay?)?)?', a:string)
    return [5, rest]
endfunction

function! Saturday(string)
    let [_, rest] = RegEx('\c\vsat(u(r(d(ay?)?)?)?)?', a:string)
    return [6, rest]
endfunction

function! January(string)
    let [_, rest] = RegEx('\c\vjan(u(a(ry?)?)?)?', a:string)
    return [1, rest]
endfunction

function! February(string)
    let [_, rest] = RegEx('\c\vfeb(r(u(a(ry?)?)?)?)?', a:string)
    return [2, rest]
endfunction

function! March(string)
    let [_, rest] = RegEx('\c\vmar(ch?)?', a:string)
    return [3, rest]
endfunction

function! April(string)
    let [_, rest] = RegEx('\c\vapr(il?)?', a:string)
    return [4, rest]
endfunction

function! May(string)
    let [_, rest] = RegEx('\c\vmay', a:string)
    return [5, rest]
endfunction

function! June(string)
    let [_, rest] = RegEx('\c\vjune?', a:string)
    return [6, rest]
endfunction

function! July(string)
    let [_, rest] = RegEx('\c\vjuly?', a:string)
    return [7, rest]
endfunction

function! August(string)
    let [_, rest] = RegEx('\c\vaug(u(st?)?)?', a:string)
    return [8, rest]
endfunction

function! September(string)
    let [_, rest] = RegEx('\c\vsep(t(e(m(b(er?)?)?)?)?)?', a:string)
    return [9, rest]
endfunction

function! October(string)
    let [_, rest] = RegEx('\c\voct(o(b(er?)?)?)?', a:string)
    return [10, rest]
endfunction

function! November(string)
    let [_, rest] = RegEx('\c\vnov(e(m(b(er?)?)?)?)?', a:string)
    return [11, rest]
endfunction

function! December(string)
    let [_, rest] = RegEx('\c\vdec(e(m(b(er?)?)?)?)?', a:string)
    return [12, rest]
endfunction

function! DaysSymbol(string)
    let [_, rest] = RegEx('\cd', a:string)
    return ['days', rest]
endfunction

function! WeeksSymbol(string)
    let [_, rest] = RegEx('\cw', a:string)
    return ['weeks', rest]
endfunction

function! TodaySymbol(string)
    let [_, rest] = RegEx('\V.', a:string)
    return ['today', rest]
endfunction

function! DefaultDateSymbol(string)
    let [_, rest] = RegEx('\V+', a:string)
    return ['default', rest]
endfunction

function! AnteMeridiem(string)
    let [_, rest] = RegEx('\cam', a:string)
    return ['am', rest]
endfunction

function! PostMeridiem(string)
    let [_, rest] = RegEx('\cpm', a:string)
    return ['pm', rest]
endfunction
" }}}
" {{{ Generic parsers combinators
function! Sequence(parsers, string) " {{{
    let rest = a:string
    let result = []
    for parser in a:parsers
	let [token, rest] = call(parser, [rest])
	let result = add(result, token)
	" l:token can have different type in the next iteration
	unlet token
    endfor
    return [result, rest]
endfunction " }}}

function! Alternative(parsers, string) " {{{
    for parser in a:parsers
	try
	    return call(parser, [a:string])
	catch /^PARSING: /
	    continue
	endtry
    endfor
    throw 'PARSING: Alternative: ' . a:string
endfunction " }}}

" Returns:
"   a pair [[], REST-OF-A:STRING] if A:TOKEN didn't match
"   a pair [[TOKENS], REST-OF-A:STRING] if A:TOKEN matched
function! Optional(token, string) " {{{
    let [tokens, rest] = Alternative([a:token, 'Empty'], a:string)

    if !empty(tokens)
	" Make one-element list out of tokens if it matched
	return [[tokens], rest]
    endif

    return [[], rest]
endfunction " }}}

" Returns: [LIST-OF-A:TOKEN, REST-OF-A:STRING]
function! Many(token, string) " {{{
    let rest = a:string
    let result = []
    try
	while 1
	    let [token, rest] = call(a:token, [rest])
	    let result = add(result, token)
	endwhile
    catch /^PARSING: /
    endtry
    return [result, rest]
endfunction " }}}

" Returns: [LIST-OF-A:TOKEN, REST-OF-A:STRING]
" Requires: a:token matches at least once
function! Many1(token, string) " {{{
    let rest = a:string
    let [match, rest] = call(a:token, [rest])
    let [tokens, rest] = Many(a:token, rest)
    return [extend([match], tokens), rest]
endfunction " }}}

" Returns: [LIST-OF-A:TOKEN, REST-OF-A:STRING]
function! SepBy(sep, token, string) " {{{
    let rest = a:string
    let tokens = []
    try
	let [match, rest] = call(a:token, [rest])
	let tokens = add(tokens, match)
	while 1
	    let [_, rest] = call(a:sep, [rest])
	    let [match, rest] = call(a:token, [rest])
	    let tokens = add(tokens, match)
	endwhile
    catch /^PARSING: /
    endtry
    return [tokens, rest]
endfunction " }}}
" }}}
" {{{ DateTime nonterminal symbols
function! DashedDate(string)
    let [tokens, rest] = Sequence(['Number', 'Dash', 'Number', 'Dash', 'Number'], a:string)
    return [{'year': tokens[0], 'month': tokens[2], 'day': tokens[4]}, rest]
endfunction

function! SlashNumber(string)
    return Sequence(['Slash', 'Number'], a:string)
endfunction

function! OptionalSlashNumber(string)
    return Optional('SlashNumber', a:string)
endfunction

function! SlashedDate(string)
    let [tokens, rest] = Sequence(['Number', 'Slash', 'Number', 'OptionalSlashNumber'], a:string)

    let result = {}
    let result.day = tokens[0]
    let result.month = tokens[2]
    if !empty(tokens[3])
	let year = tokens[3][0]
	let result.year = year[1]
    endif

    return [result, rest]
endfunction

function! DottedDate(string)
   let [tokens, rest] = Sequence(['Number', 'Dot', 'Number', 'Dot', 'Number'], a:string)
   return [{'day': tokens[0], 'month': tokens[2], 'year': tokens[4]}, rest]
endfunction

function! DayName(string)
    let [token, rest] = Alternative(['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'], a:string)
    return [{'weekday': token}, rest]
endfunction

function! MonthName(string)
    return Alternative(['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December'], a:string)
endfunction

function! SpacesNumber(string)
    return Sequence(['Spaces', 'Number'], a:string)
endfunction

function! OptionalSpacesNumber(string)
    return Optional('SpacesNumber', a:string)
endfunction

function! MonthDay(string)
    let [tokens, rest] = Sequence(['MonthName', 'Spaces', 'Number', 'OptionalSpacesNumber'], a:string)

    let result = {}
    let result.month = tokens[0]
    let result.day = tokens[2]
    if !empty(tokens[3])
	let result.year = tokens[3][0][1]
    endif

    return [result, rest]
endfunction

function! DayMonth(string)
    let [tokens, rest] = Sequence(['Number', 'Spaces', 'MonthName'], a:string)

    let result = {}
    let result.day = tokens[0]
    let result.month = tokens[2]

    return [result, rest]
endfunction

function! WeekNumber(string)
    let [tokens, rest] = Sequence(['WeeksSymbol', 'Number'], a:string)
    return [{'week': tokens[1]}, rest]
endfunction

function! YearWeekNumber(string)
    let [tokens, rest] = Sequence(['Number', 'Spaces', 'WeekNumber', 'Spaces', 'DayName'], a:string)

    let result = {}
    let result.year = tokens[0]
    let result = extend(result, tokens[2])
    let result = extend(result, tokens[4])

    return [result, rest]
endfunction

function! DashedYearWeekNumber(string)
    let [tokens, rest] = Sequence(['Number', 'Dash', 'WeekNumber', 'Dash', 'Number'], a:string)

    let result = {}
    let result.year = tokens[0]
    let result = extend(result, tokens[2])
    let result.weekday = tokens[4]

    return [result, rest]
endfunction

function! Date(string)
    return Alternative(['DashedDate', 'SlashedDate', 'DottedDate', 'DayName', 'MonthDay', 'DayMonth', 'WeekNumber', 'YearWeekNumber', 'DashedYearWeekNumber'], a:string)
endfunction

function! Meridiem(string)
    return Alternative(['AnteMeridiem', 'PostMeridiem'], a:string)
endfunction

function! OptionalMeridiem(string)
    return Optional('Meridiem', a:string)
endfunction

function! OptionalNumber(string)
    return Optional('Number', a:string)
endfunction

function! HourColonMinute(string)
    let [tokens, rest] = Sequence(['Number', 'Colon', 'Number'], a:string)
    return [{'hour': tokens[0], 'minute': tokens[2]}, rest]
endfunction

function! HourMeridiem(string)
    let [tokens, rest] = Sequence(['Number', 'Meridiem'], a:string)
    return [{'hour': tokens[0], 'meridiem': tokens[1]}, rest]
endfunction

function! HourMinuteOptionalMeridiem(string)
    let [tokens, rest] = Sequence(['HourColonMinute', 'OptionalMeridiem'], a:string)

    let result = {}
    let result = extend(result, tokens[0])
    if !empty(tokens[1])
	let result.meridiem = tokens[1][0]
    endif

    return [result, rest]
endfunction

function! HourMinuteMeridiem(string)
    return Alternative(['HourMinuteOptionalMeridiem', 'HourMeridiem'], a:string)
endfunction

function! RangeSepHourMinuteMeridiem(string)
    return Sequence(['RangeSep', 'HourMinuteMeridiem'], a:string)
endfunction

function! OptionalRangeSepHourMinuteMeridiem(string)
    return Optional('RangeSepHourMinuteMeridiem', a:string)
endfunction

function! TimeRange(string)
    let [tokens, rest] = Sequence(['HourMinuteMeridiem', 'OptionalRangeSepHourMinuteMeridiem'], a:string)

    let result = {}
    let result.start = tokens[0]
    if !empty(tokens[1])
	let result.end = tokens[1][0][1]
    endif

    return [result, rest]
endfunction

function! ColonNumber(string)
    return Sequence(['Colon', 'Number'], a:string)
endfunction

function! OptionalColonNumber(string)
    return Optional('ColonNumber', a:string)
endfunction

function! Duration(string)
    let [tokens, rest] = Sequence(['Number', 'OptionalColonNumber'], a:string)
    let result = {}
    let result.hours = tokens[0]
    if !empty(tokens[1])
	let result.minutes = tokens[1][0][1]
    endif
    return [result, rest]
endfunction

function! TimeDuration(string)
    let [tokens, rest] = Sequence(['HourMinuteMeridiem', 'Plus', 'Duration'], a:string)
    let result = tokens[0]
    let result.duration = tokens[2]
    return [result, rest]
endfunction

function! Time(string)
    return Alternative(['TimeDuration', 'TimeRange'], a:string)
endfunction

function! OptionalTime(string)
    return Optional('Time', a:string)
endfunction

function! DateOptionalTime(string)
    let [tokens, rest] = Sequence(['Date', 'Spaces', 'OptionalTime'], a:string)
    if empty(tokens[2])
	return [tokens[0], rest]
    endif
    return [extend(tokens[0], tokens[2][0]), rest]
endfunction

function! OptionalDate(string)
    return Optional('Date', a:string)
endfunction

function! TimeOptionalDate(string)
    let [tokens, rest] = Sequence(['Time', 'Spaces', 'OptionalDate'], a:string)
    if empty(tokens[2])
	return [tokens[0], rest]
    endif
    return [extend(tokens[0], tokens[2][0]), rest]
endfunction

function! DayOfMonth(string)
    let [day, rest] = Number(a:string)
    let result = {}
    let result.day = day
    return [result, rest]
endfunction

function! DateTime(string)
    return Alternative(['DateOptionalTime', 'TimeOptionalDate', 'DayOfMonth'], a:string)
endfunction

function! DateOffsetScale(string)
    return Alternative(['DaysSymbol', 'WeeksSymbol', 'DayName'], a:string)
endfunction

function! OptionalDateOffsetScale(string)
    return Optional('DateOffsetScale', a:string)
endfunction

function! DateOffset(string)
    let [tokens, rest] = Sequence(['Number', 'OptionalDateOffsetScale'], a:string)

    if empty(tokens[1])
	return [{'days': tokens[0]}, rest]
    endif

    if type(tokens[1][0]) == type({})
	return [{'weekdays': tokens[0], 'weekday': tokens[1][0]['weekday']}, rest]
    else
	if tokens[1][0] == 'weeks'
	    return [{'weeks': tokens[0]}, rest]
	elseif tokens[1][0] == 'days'
	    return [{'days': tokens[0]}, rest]
	endif
    endif
endfunction

function! RelativeFutureDateTime(string)
    let [tokens, rest] = Sequence(['Plus', 'DateOffset'], a:string)
    let tokens[1].type = 'future'
    return [tokens[1], rest]
endfunction

function! RelativePastDateTime(string)
    let [tokens, rest] = Sequence(['Minus', 'DateOffset'], a:string)
    let tokens[1].type = 'past'
    return [tokens[1], rest]
endfunction

function! RelativeDateTime(string)
    return Alternative(['RelativeFutureDateTime', 'RelativePastDateTime'], a:string)
endfunction

function! AbsoluteDateTime(string)
    let [tokens, rest] = DateTime(a:string)
    let tokens.type = 'absolute'
    return [tokens, rest]
endfunction

function! OptionalRelativeDateTime(string)
    return Optional('RelativeDateTime', a:string)
endfunction

function! TodayRelativeDateTime(string)
    let [tokens, rest] = Sequence(['TodaySymbol', 'OptionalRelativeDateTime'], a:string)

    let result = {'base': 'today'}
    if !empty(tokens[1])
	let result = extend(result, tokens[1][0])
    endif

    return [result, rest]
endfunction

function! DefaultDateRelativeDateTime(string)
    let [tokens, rest] = Sequence(['DefaultDateSymbol', 'OptionalRelativeDateTime'], a:string)

    let result = {'base': 'default'}
    if !empty(tokens[1])
	let result = extend(result, tokens[1][0])
    endif

    return [result, rest]
endfunction

function! DateTimeString(string)
    let [datetime, rest] = Alternative(['TodayRelativeDateTime', 'RelativeDateTime', 'DefaultDateRelativeDateTime', 'AbsoluteDateTime'], a:string)

    if !has_key(datetime, 'base')
	let datetime.base = 'today'
    endif

    return [datetime, rest]
endfunction

function! ParseDateTimeString(string)
    try
	return DateTimeString(a:string)
    catch /^PARSING: /
	return [{}, '']
    endtry
endfunction
" }}}
" {{{ Test cases
function! TestParseDateTimeString()
    " Examples taken from [1]

    echo ParseDateTimeString('3-2-5')
    echo ParseDateTimeString('2/5/3')
    echo ParseDateTimeString('14')
    echo ParseDateTimeString('2/5')
    echo ParseDateTimeString('Fri')
    echo ParseDateTimeString('sep 15')
    echo ParseDateTimeString('feb 15')
    echo ParseDateTimeString('sep 12 9')
    echo ParseDateTimeString('12:45')
    echo ParseDateTimeString('22 sept 0:34')
    echo ParseDateTimeString('w4')
    echo ParseDateTimeString('2012 w4 fri')
    echo ParseDateTimeString('2012-w04-5')

    echo ParseDateTimeString('+0')
    echo ParseDateTimeString('.')
    echo ParseDateTimeString('+4d')
    echo ParseDateTimeString('+2w')
    echo ParseDateTimeString('++5')
    echo ParseDateTimeString('+2tue')

    echo ParseDateTimeString('11am-1:15pm')
    echo ParseDateTimeString('11am--1:15pm')
    echo ParseDateTimeString('11am+2:15')
endfunction " }}}

" vim:foldmethod=marker

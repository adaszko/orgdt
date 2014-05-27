" References::
" [1] http://orgmode.org/manual/The-date_002ftime-prompt.html#The-date_002ftime-prompt


let s:save_cpo = &cpo
set cpo&vim


" {{{ Terminal symbols
" Returns: a pair [MATCHED-STRING, REST-OF-A:STRING]
" Throws an exception if a:string doesn't match a:regex
function! s:RegEx(regex, string)
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

function! s:Empty(string)
    return [[], a:string]
endfunction

function! s:EndOfString(string)
    return s:RegEx('\v$', a:string)
endfunction

function! s:Spaces(string)
    return s:RegEx('\v\s*', a:string)
endfunction

function! s:Colon(string)
    return s:RegEx(':', a:string)
endfunction

function! s:Dash(string)
    return s:RegEx('-', a:string)
endfunction

function! s:RangeSep(string)
    return s:RegEx('\v--?', a:string)
endfunction

function! s:Slash(string)
    return s:RegEx('/', a:string)
endfunction

function! s:Plus(string)
    return s:RegEx('\V+', a:string)
endfunction

function! s:Minus(string)
    return s:RegEx('\V-', a:string)
endfunction

function! s:Dot(string)
    return s:RegEx('\V.', a:string)
endfunction

function! s:Number(string)
    let [token, rest] = s:RegEx('\v\d+', a:string)
    return [str2nr(token), rest]
endfunction

function! s:Monday(string)
    let [_, rest] = s:RegEx('\c\vmon(d(ay?)?)?', a:string)
    return [0, rest]
endfunction

function! s:Tuesday(string)
    let [_, rest] = s:RegEx('\c\vtue(s(d(a(y?)?)?)?)?', a:string)
    return [1, rest]
endfunction

function! s:Wednesday(string)
    let [_, rest] = s:RegEx('\c\vwed(n(e(s(d(ay?)?)?)?)?)?', a:string)
    return [2, rest]
endfunction

function! s:Thursday(string)
    let [_, rest] = s:RegEx('\c\vthu(r(s(d(ay?)?)?)?)?', a:string)
    return [3, rest]
endfunction

function! s:Friday(string)
    let [_, rest] = s:RegEx('\c\vfri(d(ay?)?)?', a:string)
    return [4, rest]
endfunction

function! s:Saturday(string)
    let [_, rest] = s:RegEx('\c\vsat(u(r(d(ay?)?)?)?)?', a:string)
    return [5, rest]
endfunction

function! s:Sunday(string)
    let [_, rest] = s:RegEx('\c\vsun(d(ay?)?)?', a:string)
    return [6, rest]
endfunction

function! s:January(string)
    let [_, rest] = s:RegEx('\c\vjan(u(a(ry?)?)?)?', a:string)
    return [1, rest]
endfunction

function! s:February(string)
    let [_, rest] = s:RegEx('\c\vfeb(r(u(a(ry?)?)?)?)?', a:string)
    return [2, rest]
endfunction

function! s:March(string)
    let [_, rest] = s:RegEx('\c\vmar(ch?)?', a:string)
    return [3, rest]
endfunction

function! s:April(string)
    let [_, rest] = s:RegEx('\c\vapr(il?)?', a:string)
    return [4, rest]
endfunction

function! s:May(string)
    let [_, rest] = s:RegEx('\c\vmay', a:string)
    return [5, rest]
endfunction

function! s:June(string)
    let [_, rest] = s:RegEx('\c\vjune?', a:string)
    return [6, rest]
endfunction

function! s:July(string)
    let [_, rest] = s:RegEx('\c\vjuly?', a:string)
    return [7, rest]
endfunction

function! s:August(string)
    let [_, rest] = s:RegEx('\c\vaug(u(st?)?)?', a:string)
    return [8, rest]
endfunction

function! s:September(string)
    let [_, rest] = s:RegEx('\c\vsep(t(e(m(b(er?)?)?)?)?)?', a:string)
    return [9, rest]
endfunction

function! s:October(string)
    let [_, rest] = s:RegEx('\c\voct(o(b(er?)?)?)?', a:string)
    return [10, rest]
endfunction

function! s:November(string)
    let [_, rest] = s:RegEx('\c\vnov(e(m(b(er?)?)?)?)?', a:string)
    return [11, rest]
endfunction

function! s:December(string)
    let [_, rest] = s:RegEx('\c\vdec(e(m(b(er?)?)?)?)?', a:string)
    return [12, rest]
endfunction

function! s:DaysSymbol(string)
    let [_, rest] = s:RegEx('\cd', a:string)
    return ['days', rest]
endfunction

function! s:WeeksSymbol(string)
    let [_, rest] = s:RegEx('\cw', a:string)
    return ['weeks', rest]
endfunction

function! s:TodaySymbol(string)
    let [_, rest] = s:RegEx('\V.', a:string)
    return ['today', rest]
endfunction

function! s:DefaultDateSymbol(string)
    let [_, rest] = s:RegEx('\V+', a:string)
    return ['default', rest]
endfunction

function! s:AnteMeridiem(string)
    let [_, rest] = s:RegEx('\cam', a:string)
    return ['am', rest]
endfunction

function! s:PostMeridiem(string)
    let [_, rest] = s:RegEx('\cpm', a:string)
    return ['pm', rest]
endfunction
" }}}
" {{{ Generic parsers combinators
function! s:Sequence(parsers, string) " {{{
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

function! s:Alternative(parsers, string) " {{{
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
function! s:Optional(token, string) " {{{
    let [tokens, rest] = s:Alternative([a:token, 's:Empty'], a:string)

    if !empty(tokens)
        " Make one-element list out of tokens if it matched
        return [[tokens], rest]
    endif

    return [[], rest]
endfunction " }}}

" Returns: [LIST-OF-A:TOKEN, REST-OF-A:STRING]
function! s:Many(token, string) " {{{
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
function! s:Many1(token, string) " {{{
    let rest = a:string
    let [match, rest] = call(a:token, [rest])
    let [tokens, rest] = s:Many(a:token, rest)
    return [extend([match], tokens), rest]
endfunction " }}}

" Returns: [LIST-OF-A:TOKEN, REST-OF-A:STRING]
function! s:SepBy(sep, token, string) " {{{
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
function! s:DashedDate(string)
    let [tokens, rest] = s:Sequence(['s:Number', 's:Dash', 's:Number', 's:Dash', 's:Number'], a:string)
    return [{'year': tokens[0], 'month': tokens[2], 'day': tokens[4]}, rest]
endfunction

function! s:SlashNumber(string)
    return s:Sequence(['s:Slash', 's:Number'], a:string)
endfunction

function! s:OptionalSlashNumber(string)
    return s:Optional('s:SlashNumber', a:string)
endfunction

function! s:SlashedDate(string)
    let [tokens, rest] = s:Sequence(['s:Number', 's:Slash', 's:Number', 's:OptionalSlashNumber'], a:string)

    let result = {}
    let result.day = tokens[0]
    let result.month = tokens[2]
    if !empty(tokens[3])
        let year = tokens[3][0]
        let result.year = year[1]
    endif

    return [result, rest]
endfunction

function! s:DottedDate(string)
   let [tokens, rest] = s:Sequence(['s:Number', 's:Dot', 's:Number', 's:Dot', 's:Number'], a:string)
   return [{'day': tokens[0], 'month': tokens[2], 'year': tokens[4]}, rest]
endfunction

function! s:DayName(string)
    let [token, rest] = s:Alternative(['s:Monday', 's:Tuesday', 's:Wednesday', 's:Thursday', 's:Friday', 's:Saturday', 's:Sunday'], a:string)
    return [{'weekday': token}, rest]
endfunction

function! s:MonthName(string)
    return s:Alternative(['s:January', 's:February', 's:March', 's:April', 's:May', 's:June', 's:July', 's:August', 's:September', 's:October', 's:November', 's:December'], a:string)
endfunction

function! s:SpacesNumber(string)
    return s:Sequence(['s:Spaces', 's:Number'], a:string)
endfunction

function! s:OptionalSpacesNumber(string)
    return s:Optional('s:SpacesNumber', a:string)
endfunction

function! s:MonthDay(string)
    let [tokens, rest] = s:Sequence(['s:MonthName', 's:Spaces', 's:Number', 's:OptionalSpacesNumber'], a:string)

    let result = {}
    let result.month = tokens[0]
    let result.day = tokens[2]
    if !empty(tokens[3])
        let result.year = tokens[3][0][1]
    endif

    return [result, rest]
endfunction

function! s:DayMonth(string)
    let [tokens, rest] = s:Sequence(['s:Number', 's:Spaces', 's:MonthName'], a:string)

    let result = {}
    let result.day = tokens[0]
    let result.month = tokens[2]

    return [result, rest]
endfunction

function! s:WeekNumber(string)
    let [tokens, rest] = s:Sequence(['s:WeeksSymbol', 's:Number'], a:string)
    return [{'week': tokens[1]}, rest]
endfunction

function! s:YearWeekNumber(string)
    let [tokens, rest] = s:Sequence(['s:Number', 's:Spaces', 's:WeekNumber', 's:Spaces', 's:DayName'], a:string)

    let result = {}
    let result.year = tokens[0]
    let result = extend(result, tokens[2])
    let result = extend(result, tokens[4])

    return [result, rest]
endfunction

function! s:DashedYearWeekNumber(string)
    let [tokens, rest] = s:Sequence(['s:Number', 's:Dash', 's:WeekNumber', 's:Dash', 's:Number'], a:string)

    let result = {}
    let result.year = tokens[0]
    let result = extend(result, tokens[2])
    let result.weekday = tokens[4]

    return [result, rest]
endfunction

function! s:Date(string)
    return s:Alternative(['s:DashedDate', 's:SlashedDate', 's:DottedDate', 's:DayName', 's:MonthDay', 's:DayMonth', 's:WeekNumber', 's:YearWeekNumber', 's:DashedYearWeekNumber'], a:string)
endfunction

function! s:Meridiem(string)
    return s:Alternative(['s:AnteMeridiem', 's:PostMeridiem'], a:string)
endfunction

function! s:OptionalMeridiem(string)
    return s:Optional('s:Meridiem', a:string)
endfunction

function! s:OptionalNumber(string)
    return s:Optional('s:Number', a:string)
endfunction

function! s:HourColonMinute(string)
    let [tokens, rest] = s:Sequence(['s:Number', 's:Colon', 's:Number'], a:string)
    return [{'hour': tokens[0], 'minute': tokens[2]}, rest]
endfunction

function! s:HourMeridiem(string)
    let [tokens, rest] = s:Sequence(['s:Number', 's:Meridiem'], a:string)
    return [{'hour': tokens[0], 'meridiem': tokens[1]}, rest]
endfunction

function! s:HourMinuteOptionalMeridiem(string)
    let [tokens, rest] = s:Sequence(['s:HourColonMinute', 's:OptionalMeridiem'], a:string)

    let result = {}
    let result = extend(result, tokens[0])
    if !empty(tokens[1])
        let result.meridiem = tokens[1][0]
    endif

    return [result, rest]
endfunction

function! s:HourMinuteMeridiem(string)
    return s:Alternative(['s:HourMinuteOptionalMeridiem', 's:HourMeridiem'], a:string)
endfunction

function! s:RangeSepHourMinuteMeridiem(string)
    return s:Sequence(['s:RangeSep', 's:HourMinuteMeridiem'], a:string)
endfunction

function! OptionalRangeSepHourMinuteMeridiem(string)
    return s:Optional('s:RangeSepHourMinuteMeridiem', a:string)
endfunction

function! s:TimeRange(string)
    let [tokens, rest] = s:Sequence(['s:HourMinuteMeridiem', 'OptionalRangeSepHourMinuteMeridiem'], a:string)

    let result = {}
    let result.start = tokens[0]
    if !empty(tokens[1])
        let result.end = tokens[1][0][1]
    endif

    return [result, rest]
endfunction

function! s:ColonNumber(string)
    return s:Sequence(['s:Colon', 's:Number'], a:string)
endfunction

function! s:OptionalColonNumber(string)
    return s:Optional('s:ColonNumber', a:string)
endfunction

function! s:Duration(string)
    let [tokens, rest] = s:Sequence(['s:Number', 's:OptionalColonNumber'], a:string)
    let result = {}
    let result.hours = tokens[0]
    if !empty(tokens[1])
        let result.minutes = tokens[1][0][1]
    endif
    return [result, rest]
endfunction

function! s:TimeDuration(string)
    let [tokens, rest] = s:Sequence(['s:HourMinuteMeridiem', 's:Plus', 's:Duration'], a:string)
    let result = {}
    let result.start = tokens[0]
    let result.duration = tokens[2]
    return [result, rest]
endfunction

function! s:Time(string)
    return s:Alternative(['s:TimeDuration', 's:TimeRange'], a:string)
endfunction

function! s:OptionalTime(string)
    return s:Optional('s:Time', a:string)
endfunction

function! s:DateOptionalTime(string)
    let [tokens, rest] = s:Sequence(['s:Date', 's:Spaces', 's:OptionalTime'], a:string)
    if empty(tokens[2])
        return [tokens[0], rest]
    endif
    return [extend(tokens[0], tokens[2][0]), rest]
endfunction

function! s:OptionalDate(string)
    return s:Optional('s:Date', a:string)
endfunction

function! s:TimeOptionalDate(string)
    let [tokens, rest] = s:Sequence(['s:Time', 's:Spaces', 's:OptionalDate'], a:string)
    if empty(tokens[2])
        return [tokens[0], rest]
    endif
    return [extend(tokens[0], tokens[2][0]), rest]
endfunction

function! s:DayOfMonth(string)
    let [day, rest] = s:Number(a:string)
    let result = {}
    let result.day = day
    return [result, rest]
endfunction

function! s:DateTime(string)
    return s:Alternative(['s:DateOptionalTime', 's:TimeOptionalDate', 's:DayOfMonth'], a:string)
endfunction

function! s:DateOffsetScale(string)
    return s:Alternative(['s:DaysSymbol', 's:DayName', 's:WeeksSymbol'], a:string)
endfunction

function! s:OptionalDateOffsetScale(string)
    return s:Optional('s:DateOffsetScale', a:string)
endfunction

function! s:MakeScale(number, tokens)
    if type(a:tokens) == type({})
        return {'weeks': a:number, 'weekday': a:tokens['weekday']}
    else
        if a:tokens == 'weeks'
            return {'weeks': a:number}
        elseif a:tokens == 'days'
            return {'days': a:number}
        endif
    endif
endfunction

function! s:NumberOptionalDateOffsetScale(string)
    let [tokens, rest] = s:Sequence(['s:Number', 's:OptionalDateOffsetScale'], a:string)

    let number = tokens[0]
    let scale = tokens[1]

    if empty(scale)
        return [{'days': number}, rest]
    endif

    return [s:MakeScale(number, scale[0]), rest]
endfunction

function! s:OptionalNumberDateOffsetScale(string)
    let [tokens, rest] = s:Sequence(['s:OptionalNumber', 's:DateOffsetScale'], a:string)
    let number = empty(tokens[0]) ? 1 : tokens[0]
    return [s:MakeScale(number, tokens[1]), rest]
endfunction

function! s:DateOffset(string)
    return s:Alternative(['s:NumberOptionalDateOffsetScale', 's:OptionalNumberDateOffsetScale'], a:string)
endfunction

function! s:RelativeFutureDateTime(string)
    let [tokens, rest] = s:Sequence(['s:Plus', 's:DateOffset'], a:string)
    let tokens[1].type = 'future'
    return [tokens[1], rest]
endfunction

function! s:RelativePastDateTime(string)
    let [tokens, rest] = s:Sequence(['s:Minus', 's:DateOffset'], a:string)
    let tokens[1].type = 'past'
    return [tokens[1], rest]
endfunction

function! s:RelativeDateTime(string)
    let [tokens, rest] = s:Alternative(['s:RelativeFutureDateTime', 's:RelativePastDateTime'], a:string)

    if !has_key(tokens, 'days')
        let tokens.days = 0
    endif

    if !has_key(tokens, 'months')
        let tokens.months = 0
    endif

    if !has_key(tokens, 'years')
        let tokens.years = 0
    endif

    return [tokens, rest]
endfunction

function! s:AbsoluteDateTime(string)
    let [tokens, rest] = s:DateTime(a:string)

    let result = {'base': 'current', 'type': 'absolute'}
    let result = extend(result, tokens)

    return [result, rest]
endfunction

function! s:TodayRelativeDateTime(string)
    let [tokens, rest] = s:Sequence(['s:TodaySymbol', 's:RelativeDateTime'], a:string)

    let result = extend({'base': 'current'}, tokens[1])

    return [result, rest]
endfunction

function! s:DefaultDateRelativeDateTime(string)
    let [tokens, rest] = s:Sequence(['s:DefaultDateSymbol', 's:RelativeDateTime'], a:string)
    let result = extend({'base': 'default'}, tokens[1])
    return [result, rest]
endfunction

function! s:JustRelativeDateTime(string)
    let [tokens, rest] = s:RelativeDateTime(a:string)

    let tokens.base = 'current'

    return [tokens, rest]
endfunction

function! s:EmptyDateTime(string)
    let [_, rest] = s:EndOfString(a:string)

    let result = { 'base':      'current'
                \, 'type':      'future'
                \, 'years':     0
                \, 'months':    0
                \, 'days':      0
                \}

    return [result, rest]
endfunction

function! s:JustDefaultDateSymbol(string)
    let [_, rest] = s:DefaultDateSymbol(a:string)

    let result = { 'base':      'default'
                \, 'type':      'future'
                \, 'years':     0
                \, 'months':    0
                \, 'days':      0
                \}

    return [result, rest]
endfunction

function! s:JustTodaySymbol(string)
    let [_, rest] = s:TodaySymbol(a:string)

    let result = { 'base':      'current'
                \, 'type':      'future'
                \, 'years':     0
                \, 'months':    0
                \, 'days':      0
                \}

    return [result, rest]
endfunction

function! s:DateTimeString(string)
    return s:Alternative([ 's:TodayRelativeDateTime'
                        \, 's:DefaultDateRelativeDateTime'
                        \, 's:JustRelativeDateTime'
                        \, 's:AbsoluteDateTime'
                        \, 's:JustDefaultDateSymbol'
                        \, 's:JustTodaySymbol'
                        \, 's:EmptyDateTime'
                        \], a:string)
endfunction

function! dtparser#ParseDateTimeString(string)
    try
        return s:DateTimeString(a:string)
    catch /^PARSING: /
        return [{}, '']
    endtry
endfunction
" }}}
" {{{ Test cases
function! s:TestParseDateTimeString()
    " Examples taken from [1]

    echo dtparser#ParseDateTimeString('3-2-5')
    echo dtparser#ParseDateTimeString('2/5/3')
    echo dtparser#ParseDateTimeString('14')
    echo dtparser#ParseDateTimeString('2/5')
    echo dtparser#ParseDateTimeString('Fri')
    echo dtparser#ParseDateTimeString('sep 15')
    echo dtparser#ParseDateTimeString('feb 15')
    echo dtparser#ParseDateTimeString('sep 12 9')
    echo dtparser#ParseDateTimeString('12:45')
    echo dtparser#ParseDateTimeString('22 sept 0:34')
    echo dtparser#ParseDateTimeString('w4')
    echo dtparser#ParseDateTimeString('2012 w4 fri')
    echo dtparser#ParseDateTimeString('2012-w04-5')

    echo dtparser#ParseDateTimeString('+')
    echo dtparser#ParseDateTimeString('+0')
    echo dtparser#ParseDateTimeString('.')
    echo dtparser#ParseDateTimeString('+4d')
    echo dtparser#ParseDateTimeString('+2w')
    echo dtparser#ParseDateTimeString('++5')
    echo dtparser#ParseDateTimeString('+2tue')
    echo dtparser#ParseDateTimeString('-wed')

    echo dtparser#ParseDateTimeString('11am-1:15pm')
    echo dtparser#ParseDateTimeString('11am--1:15pm')
    echo dtparser#ParseDateTimeString('11am+2:15')
endfunction " }}}


let &cpo = s:save_cpo
unlet s:save_cpo


" vim:foldmethod=marker

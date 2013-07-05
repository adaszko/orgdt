What is it?
===========

This is a VimL port of [Org-mode-style date/time](http://orgmode.org/manual/The-date_002ftime-prompt.html#The-date_002ftime-prompt) input handling.


Status
======

Currently just a library for use in Vim scripts.


Installation
============

Assuming you have Pathogen up and running:

    cd ~/.vim/bundle
    git clone git://github.com/adaszko/orgdt.vim.git


Usage
=====

The second element of resulting list is the remaining part of string that can
be parsed further.

    :echo dtparser#ParseDateTimeString('3-2-5')
    [{'day': 5, 'base': 'today', 'year': 3, 'month': 2, 'type': 'absolute'}, '']

    :echo dtparser#ParseDateTimeString('2/5/3')
    [{'day': 2, 'base': 'today', 'year': 3, 'month': 5, 'type': 'absolute'}, '']

    :echo dtparser#ParseDateTimeString('14')
    [{'day': 14, 'base': 'today', 'type': 'absolute'}, '']

    :echo dtparser#ParseDateTimeString('2/5')
    [{'day': 2, 'base': 'today', 'month': 5, 'type': 'absolute'}, '']

    :echo dtparser#ParseDateTimeString('Fri')
    [{'base': 'today', 'type': 'absolute', 'weekday': 5}, '']

    :echo dtparser#ParseDateTimeString('sep 15')
    [{'day': 15, 'base': 'today', 'month': 9, 'type': 'absolute'}, '']

    :echo dtparser#ParseDateTimeString('feb 15')
    [{'day': 15, 'base': 'today', 'month': 2, 'type': 'absolute'}, '']

    :echo dtparser#ParseDateTimeString('sep 12 9')
    [{'day': 12, 'base': 'today', 'year': 9, 'month': 9, 'type': 'absolute'}, '']

    :echo dtparser#ParseDateTimeString('12:45')
    [{'base': 'today', 'type': 'absolute', 'start': {'minute': 45, 'hour': 12}}, '']

    :echo dtparser#ParseDateTimeString('22 sept 0:34')
    [{'day': 22, 'base': 'today', 'month': 9, 'type': 'absolute', 'start': {'minute': 34, 'hour': 0}}, '']

    :echo dtparser#ParseDateTimeString('w4')
    [{'base': 'today', 'type': 'absolute', 'week': 4}, '']

    :echo dtparser#ParseDateTimeString('2012 w4 fri')
    [{'base': 'today', 'year': 2012, 'type': 'absolute', 'week': 4, 'weekday': 5}, '']

    :echo dtparser#ParseDateTimeString('2012-w04-5')
    [{'base': 'today', 'year': 2012, 'type': 'absolute', 'week': 4, 'weekday': 5}, '']


    :echo dtparser#ParseDateTimeString('+0')
    [{'base': 'today', 'type': 'future', 'days': 0}, '']

    :echo dtparser#ParseDateTimeString('.')
    [{'base': 'today'}, '']

    :echo dtparser#ParseDateTimeString('+4d')
    [{'base': 'today', 'type': 'future', 'days': 4}, '']

    :echo dtparser#ParseDateTimeString('+2w')
    [{'base': 'today', 'type': 'future', 'weeks': 2}, '']

    :echo dtparser#ParseDateTimeString('++5')
    [{'base': 'default', 'type': 'future', 'days': 5}, '']

    :echo dtparser#ParseDateTimeString('+2tue')
    [{'base': 'today', 'weekdays': 2, 'type': 'future', 'weekday': 2}, '']


    :echo dtparser#ParseDateTimeString('11am-1:15pm')
    [{'end': {'meridiem': 'pm', 'minute': 15, 'hour': 1}, 'type': 'absolute', 'base': 'today', 'start': {'meridiem': 'am', 'hour': 11}}, '']

    :echo dtparser#ParseDateTimeString('11am--1:15pm')
    [{'end': {'meridiem': 'pm', 'minute': 15, 'hour': 1}, 'type': 'absolute', 'base': 'today', 'start': {'meridiem': 'am', 'hour': 11}}, '']

    :echo dtparser#ParseDateTimeString('11am+2:15')
    [{'meridiem': 'am', 'base': 'today', 'hour': 11, 'type': 'absolute', 'duration': {'minutes': 15, 'hours': 2}}, '']


License
=======

BSD3

What is it?
===========

This is a VimL port of [Org-mode-style date/time](http://orgmode.org/manual/The-date_002ftime-prompt.html#The-date_002ftime-prompt) input handling.


Installation
============

Assuming you have Pathogen up and running:

    cd ~/.vim/bundle
    git clone git://github.com/adaszko/org-mode-date-time.git


Usage
=====

Calling `dtparser#ParseDateTime('3-2-5')` from a Vim script will get you a dictionary of a shape easily inferred from test cases.

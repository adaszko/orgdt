if exists('g:autoloaded_orgdt') || &compatible || v:version < 700
    finish
endif

let g:autoloaded_orgdt = 1


let s:save_cpo = &cpo
set cpo&vim


function! s:get_script_id() " {{{
    return matchstr(expand('<sfile>'), '<SNR>\zs\d\+\ze_get_script_id$')
endfun " }}}

let s:script_id = s:get_script_id()

function! s:make_ref(name) " {{{
    return function(printf('<SNR>%s_%s', s:script_id, a:name))
endfunction " }}}

function! orgdt#render_date_time(dtspec) " {{{
    let g:orgdt_dtspec = a:dtspec

    python import vim
    python import sys
    " XXX
    python sys.path.insert(0, '/Users/adaszko/.vim/bundle/orgdt.vim/autoload')
    python import dtrenderer
    python dtspec = vim.vars['orgdt_dtspec']
    python dt = dtrenderer.render_date_time(dtspec)
    python vim.vars['orgdt_render_result'] = str(dt)
    let result = g:orgdt_render_result
    unlet g:orgdt_render_result
    unlet g:orgdt_dtspec

    return result
endfunction " }}}

function! s:render_hint(rendered, rest) " {{{
    if empty(a:rendered)
        return {}
    endif

    if empty(a:rest)
        return a:rendered
    endif

    return printf('%s (...%s)', a:rendered, a:rest)
endfunction " }}}

function! s:prompt_date_time_callback(input) " {{{
    let [parsed, rest] = dtparser#ParseDateTimeString(a:input)

    if parsed == {}
        return {'hint': '???'}
    endif

    let rendered = orgdt#render_date_time(parsed)
    return {'data': rendered, 'hint': s:render_hint(rendered, rest)}
endfunction " }}}

function! s:prompt_date_callback(input) " {{{
    let [parsed, rest] = dtparser#ParseDateTimeString(a:input)

    if parsed == {}
        return {'hint': '???'}
    endif

    let rendered = orgdt#render_date(parsed)
    return {'data': rendered, 'hint': s:render_hint(result, rest)}
endfunction " }}}

function! s:prompt_time_callback(input) " {{{
    let [parsed, rest] = dtparser#ParseDateTimeString(a:input)

    if parsed == {}
        return {'hint': '???'}
    endif

    let rendered = orgdt#render_time(parsed)
    return {'data': rendered, 'hint': s:render_hint(result, rest)}
endfunction " }}}

function! orgdt#prompt_date_time() " {{{
    return getline#get_line_reactively(s:make_ref('prompt_date_time_callback'))
endfunction " }}}

function! orgdt#prompt_date() " {{{
    return getline#get_line_reactively(s:make_ref('prompt_date_callback'))
endfunction " }}}

function! orgdt#prompt_time() " {{{
    return getline#get_line_reactively(s:make_ref('prompt_time_callback'))
endfunction " }}}


let &cpo = s:save_cpo
unlet s:save_cpo


" vim:foldmethod=marker

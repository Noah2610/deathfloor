nmap <leader>r :silent !RUN_TERMINAL=1 bin/run<CR>

autocmd BufNewFile,BufRead *.tsx,*.tmx setlocal syntax=xml filetype=xml
autocmd BufNewFile,BufRead *.world     setlocal syntax=json filetype=json

nmap <leader>r :silent !RUN_TERMINAL=1 bin/run<CR>

autocmd BufNewFile,BufRead *.tsx,*.tmx setlocal syntax=xml filetype=xml

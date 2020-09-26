nmap <leader>r :silent !RUN_TERMINAL=1 bin/run --features dev<CR>
nmap <leader>R :silent !RUN_TERMINAL=1 bin/run --features dev --release<CR>
nmap <leader>b :silent !RUN_TERMINAL=1 CARGO_CMD=build bin/run --features dev<CR>
nmap <leader>B :silent !RUN_TERMINAL=1 CARGO_CMD=build bin/run --features dev --release<CR>

nmap <leader>gg :!NO_COLOR=1 bin/genecs 
nmap <leader>gc :!NO_COLOR=1 bin/genecs component 
nmap <leader>gs :!NO_COLOR=1 bin/genecs system 

autocmd BufNewFile,BufRead *.tsx,*.tmx setlocal syntax=xml filetype=xml
autocmd BufNewFile,BufRead *.world     setlocal syntax=json filetype=json
autocmd BufNewFile,BufRead *.yml       setlocal shiftwidth=2 softtabstop=2 tabstop=2

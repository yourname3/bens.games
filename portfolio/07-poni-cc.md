---
title = "poni-cc"
tags = ['rust', 'compilers', 'c']

[links]
'source code' = 'https://github.com/HoneyPony/poni-cc'
---

`poni-cc` is my custom C compiler, following Nora Sandler's "Writing a C Compiler" as well as my own whims.

The main goal of the project is to write a C compiler that is about as fast (or ideally faster) than `tcc` / TinyCC. To this end, it has a single-pass frontend (no AST), as well as an internal assembler (intermediate assembly is not necessary).

In the screenshot, you can see it compiling directly to a `.o` file without an
intermediate files.
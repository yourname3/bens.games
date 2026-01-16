---
title = "PonieScript"
tags = ['rust', 'compilers', 'gamedev']

[links]
'source code' = 'https://github.com/HoneyPony/poniescript'
---

PonieScript is a custom game scripting language I am working on. It is statically typed and AOT compiled, using C as a backend. The type system and semantics are currently something like Java crossed with Rust. It is garbage collected (using a concurrent garbage collector) and supports type-safe optionals (e.g. `ClassName?`) and value types (e.g. `(int, int)`).

The compiler, language server, and garbage collector are all written in Rust.
# Julia Exploration

This directory is a bunch of [Julia](https://julialang.org/) code that
may form the basis of Mesa X.

## Why Julia?

`eval`! Julia supports dynamic code generation and `eval`. Julia
builds actual executable machine code from high level code. Further,
`eval` takes Lisp-like AST trees as a parameter. Thus, the actual formula
execution can be compiled so it's fast. I (dpp) tried to
do this with the JVM and [Integer](http://www.mozillazine.org/articles/article473.html) but didn't get too far.

Julia also has a nice type system and a REPL for very fast code exploration.


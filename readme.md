# Flylang

A programming language made to be as simple as powerful.
It includes variable definitions, objects, OOP, function declarations, if/else, and way more !
Originaly written in [NodeJS](https://github.com/Flymeth/flylang-src), I decided to re-write it in Rust to have better performances.

## The ultimate goal

With this language I don't just want to have an interpreter, or compiler. I want to have the language being compilable into any other languages, or interpreted in different maniers.

### Why ?

Because it could be cool for exemple programming language speed comparison to write a code in a single language, and by typing a command have the exact same program in different languages.

### How ?

After finishing the lexer and parser, I want the compiling part to be used trough "add-ons" (at this time I don't really know how I'll achieve that).
For exemple, you download a Flylang Add-On on a website (maybe the futur flylang's website ?), then you place it into a specific folder, and now the flylang cli will reconize it as an Add-On and with a command you can now compile your flylang program into the Add-On's one.

## RoadMap

This roadmap may change in futur (because I surely forget some features)

### Lexer

Points ending with `*` means the syntax may change in futur.

- [x] Literals (tuples ?)
- [x] Scope naming (_`@name (...)`, `@<+`, `@<positive-integer>`_)
- [x] Modifiers (_`#(modifier1, modifier2, ...) fn()`_)
- [ ] Typing syntax
- [ ] Keywords
  - [x] `if`
  - [x] `else`
  - [x] `fn`
  - [x] `cs`
  - [x] `kind`
  - [ ] `use`\* (_`use <module> [only x,y,...] [in <variable>]` - By just using `use <mod>`, all elements in `<mod>` will be global_)
  - [x] **Breakers**\*
    - [x] `stop [@loop-scope?]` : stop (like _"break"_) a loop
    - [x] `return [@fn-scope?] [value?]` : return value to a function (default value too `()`)
    - [x] `pass [@scope?]` : pass a scope (go the the end of it). This can be used to pass an `if` block, or to reloop
- [x] Loops (_`while`, `until` & `each`_)
- [x] Operations (_`+`, `-`, `_`, `\*\*`, `/`, `//`, `%`, `&`(and), `?`(or), `~`(xor), `=`(equal), `<`, `>`\_)
- [x] Variable declarations (_`:`, `::`_)
- [x] Objects/arrays (_`{`, `}`_)

### Parser

Element non implemented in the [Lexer](#lexer) is not marked in the list bellow.

- [x] Literals
- [x] Function definition
- [x] Function call
- [x] Variable declaration
- [x] Operations and priorities
- [x] Class definition
- [ ] Class instanciation
- [x] If/else
- [x] Loops
- [x] Objects (_unstable_)
- [x] Arrays
- [x] Scope naming
- [x] Modifiers
- [x] Single expression function as lambda (ex: `fn foo(true)` will always return `true` (and not `()`) because it has only 1 instruction. To avoid this, use `fn foo(true; ())`)
- [ ] Kinds (aka _traits_) (_`kind <name>(<required-traits>..., <class-kind syntax>)`_)
- [x] Breakers

### Analyser

TODO

### Optimizer

TODO

## Syntax

The goal of this language is to have a very minimalist syntax while being very powerful.
At this time I'm too busy to re-write a documentation of the language's syntax, so you could go [on my website](https://johan-janin.com/portfolio?open=flylang) to have more informations.

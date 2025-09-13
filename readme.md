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

### Lexer

Points ending with `*` means the syntax may change in futur.

- [x] Literals
- [ ] Keywords
  - [x] `if`
  - [x] `else`
  - [x] `fn`
  - [x] `cs`
  - [ ] `import`\*
  - [ ] `return`
  - [ ] `break`
  - [ ] `continue`
  - [ ] `pass`
- [x] Loops (_`while`, `until` & `for`_)\*
- [x] Operations (_`+`, `-`, `_`, `\*_`, `/`, `//`, `&`(and),`?`(or),`~`(xor),`=`(equal),`<`, `>`_)
- [x] Variable declarations (_`:`, `::`_)
- [ ] Objects/arrays (_`{`, `}`_)

This roadmap may change in futur (because I surely forget some features)

### Parser

Element non implemented in the [Lexer](#lexer) is not marked in the list bellow.

- [x] Literals
- [x] Function definition
- [x] Function call
- [x] Variable declaration
- [x] Operations and priorities
- [ ] Class definition/instanciation
- [ ] If/else
- [ ] Loops
- [ ] Objects
- [ ] Arrays

## Syntax

The goal of this language is to have a very minimalist syntax while being very powerful.
At this time I'm too busy to re-write a documentation of the language's syntax, so you could go [on my website](https://johan-janin.com/portfolio?open=flylang) to have more informations.

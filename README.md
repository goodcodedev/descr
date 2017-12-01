Descr
=====

Descr is a small language for describing other languages.
Given a description, currently it will create ast datatypes,
parser (currently through [nom](https://github.com/Geal/nom))
and a visitor trait to traverse parsed source.

"To source" generator, transform to other datastructures,
syntax highlighting definitions, some [serde](https://github.com/serde-rs/serde) integration etc
is planned.

Structs
-------
Ast structs can be described like this:
```
AstName(LPAREN ident RPAREN)
```
This will create an ast-struct like this:
```rust
pub struct AstName<'a> {
    ident: &'a str
}
```
By now the parser will recognize the following source and
parse it into the struct:
```
(some_ident)
```
Another struct could have this as a member with this addition:
```
Container(ident COLON AstName)

AstName(LPAREN ident RPAREN)
```
Which would recognize the following source:
```
container_ident: (some_ident)
```

Enums
-----
To allow alternatives, an enum can be described like this:
```
Choices {
    EQUAL => Choice1,
    LT => Choice2,
    GT => Choice3
}
```
These can be used as struct members as well:
```
StructName(left_side:ident Choices right_side:ident)
```
Here, the "ident" matches are also given a different name.

Lists
-----
Lists result in vector-members of enums or structs depending on
whether there are variations among the possibilities:
```
Source (statements)

statements:Statement[] {
    ("say" quoted) => Say,
    ("bg" Color)   => BgColor
}

(* Alternative syntax also
 * work for list items *)
Color {
    Red("red"),
    Green("green"),
    Blue("blue)
}
```
This definition would recognize the following source:
```
bg blue
say "Hello world"
```
To traverse these items, the following code could be created:
```rust
use lang::visitor::Visitor;

struct Interpr;
impl<'a> Visitor<'a> for Interpr {

    fn visit_say(&mut self, node: &'a Say) {
        println!("Saying: {}", node.quoted);
    }

    fn visit_bgcolor(&mut self, node: &'a BgColor) {
        match &node.color {
            &Red => set_bg(255, 180, 180),
            &Green => set_bg(180, 255, 180),
            &Blue => set_bg(180, 180, 255)
        };
    }
}
```

Exploration
-----------
There is a file called "playground.lang" which is for describing
a language, and a "pg-example.pg" file describing a source.
Running ```cargo run pg``` in the root will generate language
code when pg.lang is changed, and show the result of parsing
pg-example.pg otherwise.

Standard tokens
---------------
Token | Value
---|---
ident | Identifier (_alpha + alphanumeric)
string | Reads quoted string
int | Parse integer
LPAREN | (
RPAREN | )
LBRACE | {
RBRACE | }
LBRACKET | [
RBRACKET | ]
COMMA | ,
COLON | :
SEMICOLON | ;
EQUAL | =
LT | <
GT | >
LTE | <=
GTE | >=
STAR | *
EXCL | !
DOT | .
QUESTION | ?
WS | Whitespace

Things
------
- [x] "Self host"
- [ ] Recursive data structures (boxed somewhere)
- [ ] (Back) to source generator
- [ ] Include language files, maybe into context
- [ ] Annotations for things like serde integration
- [ ] "Standard library" with tokens +(?)
- [ ] Try some languages, subset of javascript, glsl
- [ ] Look for patterns to generalize
- [ ] Try to organize languages beside each other
- [ ] Transform support
- [ ] Syntax highlight generation
- [ ] Analyze rules to order by longest rule first when conflict
- [ ] Parse error messages
- [ ] Provide some language elements and type system?
- [ ] More editor support, pluggable code completion, language server?

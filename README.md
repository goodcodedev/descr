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
By now the parser will recognize the following source:
```
(some_ident)
```
And parse it into a generated struct:
```rust
pub struct AstName<'a> {
    ident: &'a str
}
```
Another struct could have this as a member:
```
Container(ident COLON AstName)

AstName(LPAREN ident RPAREN)
```
Which would recognize the following source:
```
container_ident: (some_ident)
```
And parse it into:
```
Container {
    ident: "container_ident",
    ast_name: AstName {
        ident: "some_ident"
    }
}
```

Enums
-----
To allow alternatives, an enum can be described like this:
```
Comparison {
    "=" => Equal,
    "<" => Lt,
    ">" => Gt
}
```
Which creates a simple enum struct:
```rust
Comparison {
    Equal,
    Lt,
    Gt
}
```
The variations could also have data, which generates structs for each,
and an enum container, "Alternatives":
```
Alternatives {
    IntConst(int),
    Quoted(string),
    Ident(ident)
}
```
Generates:
```rust
pub enum Alternatives {
    IntConstItem(IntConst),
    QuotedItem(Quoted),
    ...
}

pub struct IntConst {
    pub int: i32
}

pub struct Quoted<'a> {
    pub string: &'a str
}
...
```

Lists
-----
Lists result in vector-members of enums or structs depending on
whether there are variations among the possibilities:
```
Source (statements)

statements:Statement[] {
    ("say" string) => Say,
    ("bg" Color)   => BgColor
}

(* Alternative syntax also
 * work for list items *)
Color {
    Red("red"),
    Green("green"),
    Blue("blue")
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
use lang::ast::*;

struct Interpr;
impl<'a> Visitor<'a> for Interpr {

    fn visit_say(&mut self, node: &'a Say) {
        println!("Saying: {}", node.string);
    }

    fn visit_bg_color(&mut self, node: &'a BgColor) {
        use Color::*;
        match &node.color {
            &Red => set_bg(255, 180, 180),
            &Green => set_bg(180, 255, 180),
            &Blue => set_bg(180, 180, 255)
        };
    }
}
```

### Named tokens
Tokens can be given names that will resolve to member names:
```
TwoIdents(first:ident second:ident)
```
```rust
pub struct TwoIdents<'a> {
    first: &'a str,
    second: &'a str
}
```

### Optional token
A token can be made optional with a question mark:
```
Optionally("opt" ident? SEMICOLON)
```
When named, chars and tags will resolve to booleans,
other will resolve to Option.

### Until match
Parse string until token matches:
```
Until("text" parsed:!SEMICOLON)
```

See [descr.lang](https://github.com/goodcodedev/descr/blob/master/descr.lang)
for a more complete example, as well as the definition of the language.

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

Exploration
-----------
There is a file called "playground.lang" which is for describing
a language, and a "pg-example.pg" file describing a source.
Running ```cargo run pg``` in the root will generate language
code when pg.lang is changed, and show the result of parsing
pg-example.pg otherwise.

Things
------
- [x] Recursive data structures (boxed somewhere)
- [ ] Groups of tokens: Data(token1 (token2 token3)?)
- [x] (Back) to source generator
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
- [ ] Generate code for other languages (ocaml maybe) and set up serialization (just to_source?)
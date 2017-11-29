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
    Choice1(EQUAL),
    Choice2(LT),
    Choice3(GT)
}
```
These can be used as struct members as well:
```
StructName(left_side:ident CHOICES right_side:ident)
```
Here, the "ident" matches are also given a different name.

Lists
-----
Lists result in vector-members of enums or structs depending on
whether there are variations among the possibilities:
```
Source (statements)

statements:Statement[] {
    Say("say" quoted),
    BgColor("bg" Color)
}

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
To traverse this items, the following code could be created:
```rust
use lang::visitor::Visitor;
struct Interpr;
impl<'a> Visitor<'a> for Interpr {
    fn visit_say(&mut self, node: &'a Say) {
        println!("Saying: {}", node.quoted);
    }

    fn visit_bgcolor(&mut self, node: &'a BgColor) {
        match &node.color {
            Red => set_bg(255, 180, 180),
            Green => set_bg(180, 255, 180),
            Blue => set_bg(180, 180, 255)
        };
    }
}
```

Exploration
-----------
There is a folder, "pg", which contains a pg.lang file describing
a language, and a "example.pg" file describing a source.
Running ```cargo run pg``` in the root will generate language
code when pg.lang is changed, and show the result of parsing
example.pg otherwise.
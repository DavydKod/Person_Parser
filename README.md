# Person_Parser

Rust_parser

This crate is for parsing strings into Person objects.
Person structure contains the information about name(String), age(u32) and city(String).

```rust
fn to_string(&self) -> String
```

This method is implemented for Person to transform this into the String.

```rust
fn parse(string: &str) -> anyhow::Result<Person>
```

This method parse any &str into Person(under anyhow::Result<Person>). The grammar for this is written in ./grammar.pest file. Also there can be a lot of mistakes in the input &str and this parser will try to fix this and to create Person object.

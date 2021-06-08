<h1 align="center">templated-rs</h1>
A simple templating engine that can be used anywhere.

## Why yet another templating engine?
Many templates can be confusing, and sometimes they are only usable in some file formats. templated-rs is a templating engine you can use anywhere, and it has an easy to read syntax

## Building templated-rs
1. Install rust (Easiest way is to install rustup)
2. Run `cargo build --release` in the root directory.
3. Now you can find `templated-rs` in `target/release/templated-rs`

## Usage
Using templated-rs is simple, you pass any file as the first argument and you get the result in stdout.

Example:
```
$ cat example.txt
{# Here's a comment #}
{# The line below defines a variable #}
{ define foo Hello, world }
{# Let's use our variable: #}
The variable foo is { foo }!
$ templated-rs example.txt
The variable foo is Hello, world!
```

## Syntax
Here's a quick cheatsheet, until there is a proper wiki.
| Syntax               | Description           |
| -------------------- | --------------------- |
| {# ... #}            | Comment               |
| \{ hello \}          | Escapes               |
| { foo }              | Referencing variables |
| { define foo Hello } | Defining variables    |

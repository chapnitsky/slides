# Rust intro
{id: rust-intro}

## Rust resources
{id: rust-resources}

* [From Perl to Rust](https://oylenshpeegul.gitlab.io/from-perl-to-rust/introduction.html)
* [Rust Documentation](https://doc.rust-lang.org/)
* [Rust the language](https://www.rust-lang.org/)
* [Learn Rust](https://www.rust-lang.org/learn)
* [The Rust book](https://doc.rust-lang.org/book/)

## Install Rust
{id: rust-installation}

## Editor and IDE
{id: rust-editor-and-ide}

* Visual Studio Code with the `rust-analyzer` plugin.


## Hello World
{id: rust-hello-world}
{i: println!}

* `println!` is a macro (it looks like function, it generates some Rust code during compilation)

![](examples/intro/hello.rs)

```
rustc hello.rs
```

```
./hello
```

## Hello World in (immutable) variable
{id: rust-hello-world-in-immutable-variable}
{i: let}

* Variables are by default immutable

![](examples/intro/hello_world_in_immutable_variable.rs)
![](examples/intro/hello_world_in_immutable_variable.out)

## Hello World in mutable variable
{id: rust-hello-world-in-mutable-variable}
{i: let}
{i: mut}

![](examples/intro/hello_world_in_mutable_variable.rs)

## Hello World in redeclared immutable variable
{id: rust-hello-world-in-redeclared-immutable-variable}
{i: let}
{i: mut}

![](examples/intro/hello_world_in_redeclared_immutable_variable.rs)


## Hello World with Cargo
{id: hello-world-with-cargo}

```
cargo new hello
cd hello
cargo run
```

## My little Rust runner
{id: my-little-rust-runner}

This is especially usefule for the slides so I can create individual Rust example files and run them stand alone

![](rust)


## Rust and comments
{id: rust-comments}
{i: //}
{i: /* */}

* Both single-line and multi-line comments are available in Rust

![](examples/intro/comments.rs)


## Rust - Hello Foo
{id: hello-foo}

![](examples/intro/hello_foo.rs)

* [format macro](https://doc.rust-lang.org/std/fmt/)

## Rust - Hello name - input from STDIN
{id: rust-hello-name}
{i: stdin}
{i: read_line}
{i: expect}

![](examples/intro/hello_name.rs)

## Interpolation
{id: string-interpolation}

Since Rust 1.58

![](examples/intro/interpolation.rs)

## Macros
{id: macros}

Somthing that looks like a function but ends with an exclamation point. e.g. `println!`

* Metaprogramming - a program that generate a program


## Conditional: if
{id: rust-conditional-if}
{i: if}

![](examples/intro/if.rs)

## Conditional: if - else
{id: rust-conditional-if-else}

![](examples/intro/if_else.rs)

## Command line arguments - argv
{id: command-line-arguments}
{i: argv}

![](examples/intro/argv.rs)

## Variable Scope in Rust
{id: rust-variable-scope}

* Every block creates a scope

![](examples/intro/scope.rs)
![](examples/intro/scope.out)

## Rust constants
{id: rust-constants}
{i: let}

![](examples/intro/constant.rs)

## Rust mutable variables
{id: rust-variables}
{i: let}
{i: mut}

![](examples/intro/variables.rs)


## Rust rectangle ARGV
{id: rust-rectangle}

![](examples/intro/rectangle_argv.rs)

## Rust and print
{id: rust-and-print}

```
print!
println!
eprint!   to STDERR
eprintln! to STDERR

format!
```


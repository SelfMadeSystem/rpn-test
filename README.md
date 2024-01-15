# RPN Test

This is a simple RPN (Reverse Polish Notation) calculator that can be used to 
evaluate expressions in Reverse Polish Notation. Also includes an infix to 
postfix converter.

It uses the [Shunting-yard algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm) to convert infix to postfix.

## Things I might add in the future

- [ ] Compile to LLVM IR and generate an executable. That would be sick.

## Usage

> Note: [rpn/infix] refers to the rpn or infix binary.

There are three ways to use this program:

1. As a REPL (Read-Eval-Print-Loop) by running `cargo run --bin [rpn/infix]`.
2. By passing arguments to the program, e.g. `cargo run --bin rpn 1 2 +`.
> Note: You can also use quotes to pass arguments, or even mix and match. For
> example, `cargo run --bin rpn 1 2 "+ 3" 4 +` is valid.
3. By piping input to the program, e.g. `echo "1 2 +" | cargo run --bin [rpn/infix]`.
> Note: You can pipe multiple lines to the program, and it will evaluate each
> line separately. Any empty lines will be ignored and any errors will be
> printed to stderr.

## Operators and values

### Numbers

Numbers are represented as floating point numbers.

You can represent a negative number by putting a `-` directly in front of it,
e.g. `-1`.

#### Operators

The following operators are supported for numbers:

- `+` Addition
- `-` Subtraction
- `*` Multiplication
- `/` Division
- `^` Exponentiation
- `sqrt` Square root (takes one argument)

The following return a boolean:

- `=` Equality
- `!=` Inequality
- `<` Less than
- `<=` Less than or equal to
- `>` Greater than
- `>=` Greater than or equal to

### Booleans

Booleans are represented as the strings `true` and `false`.

#### Operators

The following operators are supported for booleans:

- `=` Equality/Logical XNOR
- `!=` Inequality/Logical XOR
- `&` Logical AND
- `|` Logical OR
- `!` Logical NOT (takes one argument)

## Examples

### RPN

RPN (Reverse Polish Notation) is a postfix notation for mathematical expressions.

No parentheses are needed, or even supported, since the order of operations is
determined by the order of the operands and operators.

You must include spaces between each operand and operator.

```sh
$ cargo run --bin rpn 1 2 +
3
$ cargo run --bin rpn "1 2 + 3 *" # put in quotes because * will be interpreted as a wildcard by the shell
9
$ cargo run --bin rpn "1 2 + 3 * 4 / 5 - 6 7 * +"
39.25
$ cargo run --bin rpn "2 6 + 2 * sqrt 1.5 ^"
8
$ cargo run --bin rpn "2 6 + 8 ="
true
$ cargo run --bin rpn "true false &"
false
```

### Infix

Infix notation is the standard notation for mathematical expressions.

Parentheses are supported, and are used to determine the order of operations.

Spaces aren't as required and will be automatically inserted, though my
insertion algorithm is not perfect.

```sh
$ cargo run --bin infix "1 + 2"
3
$ cargo run --bin infix "1 + 2 * -3"
-5
$ cargo run --bin infix "1 + 2 * 3 - 4 / 5"
6.2
$ cargo run --bin infix "2 + (6 + 10) ^ 1.5"
66
$ cargo run --bin infix "sqrt(10 + 6)^1.5"
8
$ cargo run --bin infix "5 * 3 < 4 * 4"
true
$ cargo run --bin infix "!((true & false) | true) != ! false" # must put space between ! and false
true
```

## License

This project is licensed under the MIT license. See the [LICENSE](LICENSE) file
for more details.

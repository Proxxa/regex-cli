# regex-cli-test

A simple command line utility for writing regex checks. Currently only matches a full string against
a pattern. Non-matching strings will return an exit code of `1`. Matching strings will return `0`.

## Installation

Simply use `cargo` to install from crates.io.

```
$ cargo install regex-cli-test
```

## Example: Matching from standard input

```
$ if echo -n "a" | regex '\w'
> then
> echo "Matches!"
> fi
Matches!
```

## Example: Standard input pattern
```
$ echo -n "\w+" | regex - hello && echo "Matches!"
Matches!
```

## Example: Verbose output

Verbose output is useful for debugging.

```
$ echo "a" | regex '\w' -v
Raw pattern: \w
Raw haystack: <none provided>
Taking haystack from standard input.
Pattern: \w
Haystack: a

Line terminator: \n
Case ignored? false
Whitespace ignored? false
Dotall? false
Multiline? false
Octal? false
Greed swapped? false
Unicode? true
Haystack does not match the specified pattern.
```
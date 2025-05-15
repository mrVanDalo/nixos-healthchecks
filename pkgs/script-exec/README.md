A small command line tool to render a line for
[nixos-healthchecks](https://github.com/mrVanDalo/nixos-healthchecks).

It prints success or failure of the given script and hides output if the exit
code is 0.

## How to test

`cargo test`

## How to update tests

`cargo insta test && cargo insta review`

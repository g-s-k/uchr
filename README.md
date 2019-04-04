# uchr: "Unicode CHaRacter"

a CLI tool to quickly find out ASCII/Unicode values for characters, or look up code points by value.

## Usage

- `uchr a` tells you that the ASCII character "a" corresponds to code point 97.
- `uchr foobar` tells you that the code points for each character in the string "foobar" are,
  respectively: 102, 111, 111, 98, 97, and 114.
- `uchr 123` tells you that code point 123 represents an opening curly brace character "{"
- `uchr 200..210` tells you that the half-open range of code points from 200 (inclusive) to 210
  (exclusive) represents the characters "ÈÉÊËÌÍÎÏÐÑ".

You can pass as many of these arguments in as desired, e.g. `uchr a foobar 277 ɘ 300..325`

## Installation

`cargo install uchr`

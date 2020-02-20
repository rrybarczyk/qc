# qc

qc (short for quick calc) is an enhanced Reverse Polish notation (RPN) command line tool designed to assist with quick and dirty calcs.

qc offers enhancements to a traditional RPN calculator including the use of a `:` before all operators that pops all items before it off the stack, performs the operation, and pushes the result onto the stack. Examples are included below.

## Installation

### Pre-built binaries

Pre-built binaries for Linux, macOS, and Windows can be found on
[the releases page](https://github.com/rrybarczyk/qc/releases).

You can use the following command to download the latest binary for Linux,
MacOS or Windows, just replace `DEST` with the directory where you'd like to
install the `qc` binary:

```sh
curl --proto '=https' --tlsv1.2 -sSf \
  https://raw.githubusercontent.com/rrybarczyk/qc/master/bin/install \
  | bash -s -- --to DEST
```

### Cargo

`qc` is written in [Rust](https://www.rust-lang.org/) and can be built from
source and installed with `cargo install qc`. To get Rust, use the
[rustup installer](https://rustup.rs/).

## Operations

### Binary Operators
- `add`     pops the top two items off the stack, adds them, and pushes the sum onto the stack
- `sub`     pops the top two items off the stack, subtracts them, and pushes the difference onto the stack
- `mul`     pops the top two items off the stack, multiplies them, and pushes the product onto the stack
- `div`     pops the top two items off the stack, divides them, and pushes the quotient onto the stack
- `:<binary operator>` pops all previous items off the stack, folds the values of the stack together using binary operator, and pushes the result onto the stack

### Unary Operators
- `.` pops the top item off the stack and prints it
- `pop` pops the top item off the stack
- `endian`  pops the top item off of the stack, swaps the endianness, and pushes the result onto the stack
- `:<unary operator>` maps the unary operator over the current stack items

## Examples
### Traditional RPN Capabilities
#### Addition
```
$ qc 1 2 add
Script                  Stack                   Details

                        [ ]
1                       [ 1 ]
2                       [ 1 2 ]
add                     [ 3 ]                   1 + 2


$ qc 1 2 3 add add
Script                  Stack                   Details
                        [ ]
1                       [ 1 ]
2                       [ 1 2 ]
3                       [ 1 2 3 ]
add                     [ 1 5 ]                  2 + 3
add                     [ 6 ]                    1 + 5
```

#### Subtraction
```
$ qc 1 2 sub
Script                  Stack                   Details
                        [ ]
1                       [ 1 ]
2                       [ 1 2 ]
sub                     [ -1 ]                  1 - 2

$ qc 1 2 3 sub sub
Script                  Stack                   Details
                        [ ]
1                       [ 1 ]
2                       [ 1 2 ]
3                       [ 1 2 3 ]
sub                     [ 1 -1 ]                2 - 3
sub                     [ 2 ]                   1 - -1
```

#### Multiplication
```
$ qc 1 2 mul
Script                  Stack                   Details

                        [ ]
1                       [ 1 ]
2                       [ 1 2 ]
mul                     [ 2 ]                   1 * 2


$ qc 1 2 3 mul mul
Script                  Stack                   Details
                        [ ]
1                       [ 1 ]
2                       [ 1 2 ]
3                       [ 1 2 3 ]
mul                     [ 1 6 ]                 2 * 3
mul                     [ 6 ]                   1 * 6
```

#### Division
```
$ qc 3 9 div
Script                  Stack                   Details

                        [ ]
3                       [ 3 ]
9                       [ 3 9 ]
div                     [ 0.5 ]                 3 / 9


$ qc 1 2 3 div div
Script                  Stack                   Details
                        [ ]
1                       [ 1 ]
2                       [ 1 2 ]
3                       [ 1 2 3 ]
div                     [ 1 6 ]                 2 * 3
div                     [ 6 ]                   6 * 1
```

#### Miscellaneous

```
$ qc 4 7 9 add 2 8 mul mul mul
Script                  Stack                   Details
4                       [ 4 ]
7                       [ 4 7 ]
9                       [ 4 7 9 ]
add                     [ 4 16 ]                7 + 9
2                       [ 4 16 2]
8                       [ 4 16 2 8 ]
mul                     [ 4 16 16 ]             2 * 8
mul                     [ 4 256 ]               16 * 16
mul                     [ 1024 ]                4 * 256
```

```
$ qc 4 7 9 add 2 8 mul mul 4 div sub
Script                  Stack                   Details
4                       [ 4 ]
7                       [ 4 7 ]
9                       [ 4 7 9 ]
add                     [ 4 16 ]                7 + 9
2                       [ 4 16 2]
8                       [ 4 16 2 8 ]
mul                     [ 4 16 16 ]             2 * 8
mul                     [ 4 256 ]               16 * 16
4                       [ 4 256 4]
div                     [ 4 64 ]                256 / 4
sub                     [ -60 ]                 4 - 64
```

```
$ qc 4 7 9 add add  2 3 5 mul mul mul 1 1 sub sub 20 5 div div .
Script                  Stack                   Details
4                       [ 4 ]
7                       [ 4 7 ]
9                       [ 4 7 9 ]
add                     [ 4 16 ]                7 + 9
add                     [ 20 ]                  4 + 16
2                       [ 20 2 ]
3                       [ 20 2 3 ]
5                       [ 20 2 3 5 ]
mul                     [ 20 2 15 ]             3 * 5
mul                     [ 20 30 ]               2 * 15
mul                     [ 600 ]                 20 * 30
1                       [ 600 1 ]
1                       [ 600 1 1 ]
sub                     [ 600 0 ]               600 - 1
sub                     [ 600 ]                 600 - 0
20                      [ 600 20 ]
5                       [ 600 20 5 ]
div                     [ 600 4 ]               20 / 5
div                     [ 150 ]                 600 / 4

> dec: 150        hex: 0x96          oct: o226        bin: b10010110
```

### Enhanced RPN Capabilities
```
$ qc 4 7 9 :add 2 3 5 :mul 1 1 :sub 20 5 :div .
Script                  Stack                   Details
4                       [ 4 ]
7                       [ 4 7 ]
9                       [ 4 7 9 ]
:add                    [ 20 ]                  4 + 7 + 9
2                       [ 20 2 ]
3                       [ 20 2 3 ]
5                       [ 20 2 3 5 ]
:mul                    [ 600 ]                 20 * 2 * 3 * 5
1                       [ 600 1 ]
1                       [ 600 1 1 ]
: sub                   [ 600 ]                 600 - (1 - 1)
20                      [ 600 20 ]
5                       [ 600 20 5 ]
:div                    [ 150 ]                 600 / (20 / 5)

> dec: 150        hex: 0x96          oct: o226        bin: b10010110

$ qc 0xa xb o22 b1101 :.
> dec: 13         hex: 0xd           oct: o15         bin: b1101
> dec: 18         hex: 0x12          oct: o22         bin: b10010
> dec: 11         hex: 0xb           oct: o13         bin: b1011
> dec: 10         hex: 0xa           oct: o12         bin: b1010
```

### Printing
Pop of the top item on the stack and print it.
```
$ qc 1 2 3 add add .
> dec: 6          hex: 0x6           oct: o6          bin: b110

$ qc 32 8 div .
> dec: 4          hex: 0x4           oct: o4          bin: b100

$ qc 1 2 3 :add 4 5 :mul .
> dec: 120        hex: 0x78          oct: o170        bin: b1111000


$ qc 0xa . xb . o22 . b1101 .
> dec: 10         hex: 0xa           oct: o12         bin: b1010
> dec: 11         hex: 0xb           oct: o13         bin: b1011
> dec: 18         hex: 0x12          oct: o22         bin: b10010
> dec: 13         hex: 0xd           oct: o15         bin: b1101

$ qc 0xa xb o22 b1101 :.
> dec: 13         hex: 0xd           oct: o15         bin: b1101
> dec: 18         hex: 0x12          oct: o22         bin: b10010
> dec: 11         hex: 0xb           oct: o13         bin: b1011
> dec: 10         hex: 0xa           oct: o12         bin: b1010
```

## TODO
- [x] Hexadecimal, octal, and binary formatted print
- [ ] Endianness swapping on all stack items plus formatted print
```
$ qc 0xe803000000000000 0xd007000000000000 :endian
> dec: 1000     hex: 0x00000000000003e8     oct: o1750      bin: b01111101000
> dec: 2000     hex: 0x07d0000000000000     oct: o3720      bin: b11111010000
```
- [ ] Hashing on all stack items
```
qc 0x00000000000003e8 0x00000000000007d0 :sha256d
```
- [ ] Concatenate all stack items and hash
```
qc 0x00000000000003e8 0x00000000000007d0 :cat sha256d
```


## Character Guide
Safe
```
SPACE
ABCDEFGHIJKLMNOPQRSTUVWXYZ
abcdefghijklmnopqrstuvwxyz
0123456789
@[]^_{}+,-./:
```

Dangerous
```
\`|~<>"'#&()*;? TAB
```


Questionable
```
{}!%=
~ - can use if not first character in a line
$ - on its own can be an operator, but not in fish
% - used for job control than zsh
= _ zsh ? is not ok
```

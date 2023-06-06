# stcalc

## Stack-based Calculator

Calculate stacking and unstacking values.

## Installation

```sh
cargo install stcalc
```

## Commands

- Floating point numbers are pushed into the stack top
- `+` unstack adding
- `-` invert stack top signal
- `*` unstack multiplying
- `/` invert stack top
- `!` discard stack top
- `=` show stack top as a number
- `.` show stack top as a character

### Example:

```sh
echo '33 100 108 3 24 55 32 3 7 29 72 . + . + . . + . ! . + . + . + . ! . ! . ! . .' | stcalc
```

License: BSD-3-Clause

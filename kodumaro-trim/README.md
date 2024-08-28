# kodumaro-trim

Kodumaro Trim aims to be a shell in-pipe tool for removing leading and trailing characters from strings.

## Installation guide

```sh
cargo install kodumaro-trim
```

## Usage

```sh
Usage: trim [OPTIONS] [FILE]

Arguments:
  [FILE]  input file, defaults to STDIN

Options:
  -l, --left         trim only leading
  -r, --right        trim only trailing
  -c, --char <CHAR>  character to be removed, defaults to whitespaces
  -h, --help         Print help
```

### Examples

```sh
echo '   hello world   ' | trim
```

License: BSD-3-Clause

# Kodumaro HTTP CLI

Kodumaro HTTP CLI inspired by [HTTPie][].

## Project status

It’s still a kinda-usable work in progress.


## Installation

```sh
cargo +nightly install kodumaro-http-cli
```


## Usage

```
Usage: http <COMMAND>

Commands:
  connect  performs a CONNECT request
  delete   performs a DELETE request
  get      performs a GET request
  head     performs a HEAD request
  option   performs a OPTION request
  patch    performs a PATCH request
  post     performs a POST request
  put      performs a PUT request
  trace    performs a TRACE request
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Try `http help <COMMAND>` for more information.


## TODO

- Support `multipart/form-data`


[HTTPie]:  https://httpie.io/

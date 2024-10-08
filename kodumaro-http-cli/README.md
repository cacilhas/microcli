# Kodumaro HTTP CLI

## Installation

```sh
cargo +nightly install kodumaro-http-cli
```

## Usage

```
Usage: http [OPTIONS] <COMMAND>

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
  -j, --json                           data items from the command line are serialized as a JSON object
  -f, --form                           data items from the command line are serialized as form fields
      --multipart                      similar to --form, but always sends a multipart/form-data request (i.e., even without files)
      --boundary <BOUNDARY>            specifies a custom boundary string for multipart/form-data requests
      --raw <RAW>                      allows you to pass raw request data without extra processing
  -o, --output <OUTPUT>                save output to file instead of stdout
  -d, --download                       do not print the response body to stdout; Rather, download it and store it in a file
  -a, --auth <AUTH>                    basic authentication (user[:password])
  -F, --follow                         follows Location redirects
      --max-redirects <MAX_REDIRECTS>  by default, requests have a limit of 30 redirects [default: 30]
      --verify <VERIFY>                set to "no" (or "false") to skip checking the host's SSL certificate; defaults to "yes" ("true") [default: yes]
  -v, --verbose                        Show headers
  -h, --help                           Print help
  -V, --version                        Print version
```

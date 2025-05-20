# Kodumaro HTTP CLI

Kodumaro HTTP CLI inspired by [HTTPie][].


## Project status

It’s still a kinda-usable work in progress.


## Installation

```shell
cargo install kodumaro-http-cli
```


## Usage

```
Usage: http <COMMAND> [OPTIONS] <URL> [PARAMS]...

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
      --raw <RAW>                      allows you to pass raw request data without extra processing
  -o, --output <OUTPUT>                save output to file instead of stdout [default: URL path file name]
  -d, --download                       do not print the response body to stdout; rather, download it and store it in a file [env: HTTP_DOWNLOAD=]
  -a, --auth <AUTH>                    basic authentication (user[:password]) or bearer token [env: HTTP_AUTH=]
  -F, --follow                         follows Location redirects [env: HTTP_FOLLOW=]
      --max-redirects <MAX_REDIRECTS>  when following redirects, max redirects [env: HTTP_MAX_REDIRECTS=10] [default: 30]
      --verify <VERIFY>                set to "no" to skip checking the host's SSL certificate [env: HTTP_VERIFY=no] [default: yes]
      --fail                           fail on error status code [env: HTTP_FAIL=]
  -v, --verbose                        Show headers [env: HTTP_VERBOSE=true]

Arguments:
  <URL>        the URL to connect to
  [PARAMS]...  header:value, querystring==value, and/or payload=value; @value means value from file content, str!value force value to be string
```

### Environment variables

- `HTTP_DOWNLOAD=true`: enable `--download`
- `HTTP_AUTH=<auth>`: use `<auth>` as authentication (`--auth=<auth>`)
- `HTTP_FOLLOW=true`: enable `--follow`
- `HTTP_MAX_REDIRECTS=<num>`: set max redirects to `<num>` (`--max-redirects=<num>`)
- `HTTP_VERIFY=no`: disable SSL certificate verification (`--verify=no`)


## TODO

- Support `multipart/form-data`
- Support configuration file


[HTTPie]:  https://httpie.io/

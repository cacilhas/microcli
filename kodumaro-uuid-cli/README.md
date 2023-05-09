[RFC 4122]: https://www.rfc-editor.org/rfc/rfc4122
[The 3-Clause BSD License]: https://opensource.org/license/bsd-3-clause/

# UUID CLI

UUID generator ([RFC 4122]).

This is a command line tool, **do not** install it using `cargo add`!!

## Installation guide

You need to enable `uuid_unstable` configuration flag:

```sh
RUSTFLAGS='--cfg uuid_unstable' cargo install kodumaro-uuid-cli
```

It’s gonna create a `~/.cargo/bin/uuid` executable.

## Usage

```sh
Usage: uuid [COMMAND]

Commands:
  nil   generate nil UUID
  v1    generate UUIDv1, time-based UUID
  v3    generate UUIDv3, name-based MD5 UUID
  v4    generate UUIDv4, random UUID
  v5    generate UUIDv5, name-based SHA1 UUID
  v6    generate UUIDv6, field-compatible version of UUIDv1
  v7    generate UUIDv7, Unix Epoch timestamp-based UUID
  v8    generate UUIDv8, vendor-specific UUID
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## License

- [The 3-Clause BSD License]

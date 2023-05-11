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

### Examples

```sh
$ uuid
urn:uuid:4db78d44-e170-42a3-bf93-418b9baeae2b

$ uuid help
UUID generator (RFC 4122), see <https://www.rfc-editor.org/rfc/rfc4122>

Usage: uuid [COMMAND]

Commands:
  nil   generate nil UUID
  v1    generate UUIDv1, time-based UUID
  v3    generate UUIDv3, name-based MD5 UUID
  v4    generate UUIDv4, random UUID
  v5    generate UUIDv5, name based SHA1 UUID
  v6    generate UUIDv6, field-compatible version of UUIDv1
  v7    generate UUIDv7, Unix Epoch timestamp-based UUID
  v8    generate UUIDv8, vendor-specific UUID
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

$ uuid nil
urn:uuid:00000000-0000-0000-0000-000000000000

$ uuid v1
urn:uuid:xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx

$ uuid v3 $(uuid v7) test
urn:uuid:5604097f-ffa0-3934-9635-cb03308240fe

$ uuid v5 $(uuid v7) test
urn:uuid:d8beedbe-ca82-57ef-8dc1-ca501caeb151

$ uuid v6 blabla
urn:uuid:1edee6f2-13ee-6741-b1dd-626c61626c61

$ uuid v7
urn:uuid:018800be-993e-7990-b64a-900ba7dd54e3

$ uuid v8 'Some long data!'
urn:uuid:536f6d65-206c-8f6e-a720-646174612100

$ uuid help v8
generate UUIDv8, vendor-specific UUID

Usage: uuid v8 <METADATA>

Arguments:
  <METADATA>  vendor’s metadata to be encoded into the UUIDv8, up to 16 bytes

Options:
  -h, --help  Print help
```

## License

- [The 3-Clause BSD License]

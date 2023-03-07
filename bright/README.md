# `brightcli`

Util to change monitor brightness.

## Build and install

```sh
cd bright/
cargo build --release
install --mode=755 target/release/brightcli /usr/local/bin/
```

## Use

- `brightcli`: return current brightness
- `brightcli +`: increment brightness and return the new value
- `brightcli -`: decrement brightness and return the new value

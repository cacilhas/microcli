# `bright`

Util to change monitor brightness.

## Build and install

```sh
cd bright/
cargo build --release
install --mode=755 target/release/bright /usr/local/bin/
```

## Use

- `bright`: return current brightness
- `bright +`: increment brightness and return the new value
- `bright -`: decrement brightness and return the new value

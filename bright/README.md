# `brightcli`

Util to change monitor brightness.

## Build and install

```sh
cargo install brightcli

# Whithout the following commands, brightcli will be read-only:
sudo chown root ~/.cargo/bin/brightcli
sudo chmod u+s  ~/.cargo/bin/brightcli
```

## Use

- `brightcli`: return current brightness
- `brightcli +`: increment brightness and return the new value
- `brightcli -`: decrement brightness and return the new value

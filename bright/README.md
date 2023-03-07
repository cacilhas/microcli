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

- `brightcli`: returns current brightness
- `brightcli +`: increments brightness and returns its new value
- `brightcli -`: decrements brightness and returns its new value

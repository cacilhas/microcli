# `lidwatch`

Util to watch LID switching.

## Build and install

```sh
cd lidwatch/
cargo build --release
install --mode=755 target/release/lidwatch /usr/local/bin/
```

## Use

```sh
lidwatch /dev/input/<lid event> <command to be executed on LID closed>
```

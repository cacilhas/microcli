# `gitbranch`

It simply shows current branch name - or nothing if it isn’t a git repo.

## Build and install

```sh
cd gitbranch/
cargo build --release
install --mode=755 target/release/gitbranch /usr/local/bin/
```

## Use

- `gitbranch <directory>`

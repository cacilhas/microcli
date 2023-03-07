[golang]: https://golang.org/
[lazydocker]: https://github.com/jesseduffield/lazydocker
[Sakura]: https://www.linuxlinks.com/sakura/

# Docker monitor

Shows how many containers are running.

## Install

For the current user:

```sh
make install
```

Into the system:

```sh
make
sudo make PREFIX=/usr/local install
```
### Dependencies

- [Go][golang] (for compiling)
- [`lazydocker`][lazydocker]

## Execute

Run `~/.local/share/dockermon/dockermon`. To use another manager, for instance
[Sakura][], pass the command as parameter:

```sh
~/.local/share/dockermon/dockermon sakura -e lazydocker
```

## Uninstall

Remove `~/.local/share/dockermon` directory.

If you have installed into the system, remove `$PREFIX/dockermon` directory.

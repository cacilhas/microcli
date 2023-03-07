[golang]: https://golang.org/
[i3wm]: https://i3wm.org/
[lazydocker]: https://github.com/jesseduffield/lazydocker
[Sakura]: https://www.linuxlinks.com/sakura/

# MicroCLI

Bunch of random CLI tools.

-----

## `bright`

Util to change monitor brightness.

### Use

- `bright`: return current brightness
- `bright +`: increment brightness and return the new value
- `bright -`: decrement brightness and return the new value

-----

## Docker monitor

Shows how many containers are running.

### Install

For the current user:

```sh
make install
```

Into the system:

```sh
make
sudo make PREFIX=/usr/local install
```

#### Dependencies

- [Go][golang] (for compiling)
- [`lazydocker`][lazydocker]

### Execute

Run `~/.local/share/dockermon/dockermon`. To use another manager, for instance
[Sakura][], pass the command as parameter:

```sh
~/.local/share/dockermon/dockermon sakura -e lazydocker
```

### Uninstall

Remove `~/.local/share/dockermon` directory.

If you have installed into the system, remove `$PREFIX/dockermon` directory.

-----

## `gitbranch`

It simply shows current branch name - or nothing if it isn’t a git repo.

### Use

- `gitbranch <directory>`

-----

## `i3quitdialog`

A simple quit dialog for [i3wm][].

-----

## `lidmonitor`

Util to montor LID switching.

### Use

```
lidmonitor /dev/input/<lid event> <command to be executed on LID closed>
```

-----

## Stack-based Calculator

Calculate stacking and unstacking values.

### Commands

- Float number
- `+`
- `-`
- `*`
- `/`
- `^` (raising)
- `=` (show numerical result)
- `!` (print character)

-----

## Copyright

- ©2021 [Rodrigo Cacilhας](mailto:montegasppa@cacilhas.info).
- [BSD-3 Clause License](COPYING).

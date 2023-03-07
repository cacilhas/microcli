[i3wm]: https://i3wm.org/

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

©2021-2023 [Rodrigo Cacilhας](mailto:montegasppa@cacilhas.info).

All codes in this repository are licensed under the
[BSD-3 Clause License](COPYING), except those directories containing  their own
`COPYING` file.

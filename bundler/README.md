[config nu]: https://www.nushell.sh/commands/docs/config_nu.html
[Nushell]: https://www.nushell.sh/

# Bundler

A software package bundler for [Nushell][].

## Installation

**After** the following procedures, run [`config nu`][config nu] and add the
following line to the configuration file:

```nu
use bundler.nu
```

### Installing on ArchLinux

Using Bash:

```sh
curl -o ~/.config/nushell/scripts/bundler.nu https://raw.githubusercontent.com/cacilhas/microcli/master/bundler/archlinux.nu
```

Using Nushell:

```nu
http get -r https://raw.githubusercontent.com/cacilhas/microcli/master/bundler/archlinux.nu out> ~/.config/nushell/scripts/bundler.nu
```

### Installing on Debian GNU/Linux

TODO

### Installing on macOS

TODO

## Use

Run:

```nu
bundler ?
```

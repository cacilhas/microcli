[Apt]: https://wiki.debian.org/Apt
[bat]: https://crates.io/crates/bat
[config nu]: https://www.nushell.sh/commands/docs/config_nu.html
[dpkg]: https://manpages.debian.org/stretch/dpkg/dpkg.1.en.html
[fzf]: https://github.com/junegunn/fzf
[Homebrew]: https://brew.sh/
[less]: https://www.greenwoodsoftware.com/less/
[lv]: https://manpages.debian.org/testing/lv/pager.1.en.html
[Nushell]: https://www.nushell.sh/
[Pacman]: https://wiki.archlinux.org/title/Pacman
[sudo]: https://www.sudo.ws/
[Yum]: https://www.redhat.com/sysadmin/how-manage-packages

# Bundler

A software package bundler for [Nushell][].

## Requirements

- [`fzf`][fzf]
- [`sudo`][sudo]
- a pager (one of):
  - [`bat`][bat]
  - [`lv`][lv]
  - [`less`][less]
  - `more`
- your system’s package manager (one of):
  - [Pacman][]
  - [Apt][] and [`dpkg`][dpkg]
  - [Yum][]
  - [Homebrew][]

## Installation

**After** the procedures below, run [`config nu`][config nu] and add the
following line to the configuration file:

```nu
use bundler.nu
```

### Installing on `pacman`-based distros (ArchLinux and derivatives)

Using Bash:

```sh
curl -o ~/.config/nushell/scripts/bundler.nu https://raw.githubusercontent.com/cacilhas/microcli/master/bundler/archlinux.nu
```

Using Nushell:

```nu
http get -r https://raw.githubusercontent.com/cacilhas/microcli/master/bundler/archlinux.nu out> ~/.config/nushell/scripts/bundler.nu
```

### Installing on Apt-based distros (Debian GNU/Linux and derivatives)

TODO

### Installing on RPM-based distros (Fedora Core, CentOS, and equivalents)

TODO

### Installing on macOS

Using Bash:

```sh
curl -o '~/Library/Application Support/nushell/scripts/bundler.nu' https://raw.githubusercontent.com/cacilhas/microcli/master/bundler/macos.nu
```

Using Nushell:

```nu
http get -r https://raw.githubusercontent.com/cacilhas/microcli/master/bundler/macos.nu out> '~/Library/Application Support/nushell/scripts/bundler.nu'
```

## Use

Run:

```nu
bundler ?
```

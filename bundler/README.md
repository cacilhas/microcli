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
[Scoop]: https://scoop.sh/
[sudo]: https://www.sudo.ws/
[Yum]: https://www.redhat.com/sysadmin/how-manage-packages

# Bundler

A software package bundler for [Nushell][].

-----

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
  - [Scoop][]

-----

## Installation

**After** the procedures below, run [`config nu`][config nu] and add the
following line to the configuration file:

```nu
use bundler.nu
```

### Installing on Pacman-based distros (ArchLinux and derivatives)

Using Bash:

```sh
bundler_file=$XDG_CONFIG_HOME/nushell/scripts/bundler.nu
curl -o $bundler_file  https://raw.githubusercontent.com/cacilhas/microcli/master/bundler/archlinux.nu
```

Using Nushell:

```nu
let bundler_file = ([$env.XDG_CONFIG_HOME nushell scripts bundler.nu] | path join)
http get -r https://raw.githubusercontent.com/cacilhas/microcli/master/bundler/archlinux.nu | save $bundler_file
```

### Installing on Apt-based distros (Debian GNU/Linux, Ubuntu, and derivatives)

**Not fully tested.**

Using Bash:

```sh
bundler_file=$XDG_CONFIG_HOME/nushell/scripts/bundler.nu
curl -o $bundler_file  https://raw.githubusercontent.com/cacilhas/microcli/master/bundler/debian.nu
```

Using Nushell:

```nu
let bundler_file = ([$env.XDG_CONFIG_HOME nushell scripts bundler.nu] | path join)
http get -r https://raw.githubusercontent.com/cacilhas/microcli/master/bundler/debian.nu | save $bundler_file
```

### Installing on RPM-based distros (Fedora Core, CentOS, and equivalents)

**Not tested.**

Using Bash:

```sh
bundler_file=$XDG_CONFIG_HOME/nushell/scripts/bundler.nu
curl -o $bundler_file  https://raw.githubusercontent.com/cacilhas/microcli/master/bundler/centos.nu
```

Using Nushell:

```nu
let bundler_file = ([$env.XDG_CONFIG_HOME nushell scripts bundler.nu] | path join)
http get -r https://raw.githubusercontent.com/cacilhas/microcli/master/bundler/centos.nu | save $bundler_file
```

### Installing on macOS

**Not fully tested.**

Using Bash:

```sh
bundler_file="$HOME/Library/Application Support/nushell/scripts/bundler.nu"
curl -o $bundler_file https://raw.githubusercontent.com/cacilhas/microcli/master/bundler/macos.nu
```

Using Nushell:

```nu
let bundler_file = ([$nu.home-path Library 'Application Support' nushell scripts bundler.nu] | path join)
http get -r https://raw.githubusercontent.com/cacilhas/microcli/master/bundler/macos.nu | save $bundler_file
```

### Installing on Windows

**Not tested. Note: not sure about `bundler.nu` path.**

Using PowerShell:

```powershell
$BundlerFile = "$HOME/.nushell/scripts/bundler.nu"
Invoke-WebRequest -Uri https://raw.githubusercontent.com/cacilhas/microcli/master/bundler/windows.nu -OutFile $BundlerFile
```

Using Nushell:

```nu
let bundler_file = ([$nu.home-path .nushell scripts bundler.nu] | path join)
http get -r https://raw.githubusercontent.com/cacilhas/microcli/master/bundler/windows.nu | save $bundler_file
```

-----

## Use

Run:

```nu
bundler ?
```

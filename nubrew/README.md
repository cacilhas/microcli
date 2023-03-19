[Apt]: https://wiki.debian.org/Apt
[bat]: https://crates.io/crates/bat
[config nu]: https://www.nushell.sh/commands/docs/config_nu.html
[dpkg]: https://manpages.debian.org/stretch/dpkg/dpkg.1.en.html
[fzf]: https://github.com/junegunn/fzf
[less]: https://www.greenwoodsoftware.com/less/
[lv]: https://manpages.debian.org/testing/lv/pager.1.en.html
[Nushell]: https://www.nushell.sh/
[Pacman]: https://wiki.archlinux.org/title/Pacman
[Scoop]: https://scoop.sh/
[sudo]: https://www.sudo.ws/
[Yum]: https://www.redhat.com/sysadmin/how-manage-packages

# NuBrew

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
  - [Scoop][]

-----

## Installation

**After** the procedures below, run [`config nu`][config nu] and add the
following line to the configuration file:

```nu
use brew.nu
```

### Installation on Linux distros using Nushell

The `brew_url` depends on your distro:

- Pacman-based: `https://raw.githubusercontent.com/cacilhas/microcli/master/brew/archlinux.nu`
- Apt-based: `https://raw.githubusercontent.com/cacilhas/microcli/master/brew/archlinux.nu`
- RPM-based: `https://raw.githubusercontent.com/cacilhas/microcli/master/brew/centos.nu`

```nu
let brew_file = ([$env.XDG_CONFIG_HOME nushell scripts brew.nu] | path join)
http get -r $brew_url | save $brew_file
```

### Installation on Linux distros using Bash

The `brew_url` depends on your distro:

- Pacman-based: `https://raw.githubusercontent.com/cacilhas/microcli/master/brew/archlinux.nu`
- Apt-based: `https://raw.githubusercontent.com/cacilhas/microcli/master/brew/archlinux.nu`
- RPM-based: `https://raw.githubusercontent.com/cacilhas/microcli/master/brew/centos.nu`

```sh
brew_file=$XDG_CONFIG_HOME/nushell/scripts/brew.nu
curl -o $brew_file $brew_url
```

### Installing on Windows

**Not tested. Note: not sure about `brew.nu` path.**

Using Nushell:

```nu
let brew_file = ([$nu.home-path .nushell scripts brew.nu] | path join)
http get -r https://raw.githubusercontent.com/cacilhas/microcli/master/brew/windows.nu | save $brew_file
```

Using PowerShell:

```powershell
$BundlerFile = "$HOME\.nushell\scripts\brew.nu"
Invoke-WebRequest -Uri https://raw.githubusercontent.com/cacilhas/microcli/master/brew/windows.nu -OutFile $BundlerFile
```

-----

## Use

```nu
brew ?
```

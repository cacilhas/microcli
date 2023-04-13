[blocklet]: https://vivien.github.io/i3blocks/#_i3blocks_properties
[i3wm]: https://i3wm.org/

# I3 Quit Dialog

A simple quit dialog for [i3wm][].

## Build and install

```sh
cargo install i3quitdialog
```

## Settings

The “`Exit`” button works out of the box. In order to allow users to halt and
reboot the system, add them to the `power` group.

You can just call it:

```sh
i3quitdialog
```

However, if you are setting a [blocklet][] to launch I3 Quit Dialog, prefer
using `i3-msg`:

```ini
[Logoff]
command=i3-msg -q -- exec --no-startup-id i3quitdialog
color=#ff0000
full_text=⏻
text=⏻
interval=0
#signal=10  # uncomment this line to capture `pkill -SIGRTMIN+10 i3blocks`
```

### `.XResources` support

I3 Quit Dialog supports the following keys from `xrdb` (case sensitive):

- `i3quitdialog.Title.foreground`: window colour
- `i3quitdialog.Title.background`: window background colour
- `i3quitdialog.ExitButton.color`: exit button colour
- `i3quitdialog.CancelButton.color`: cancel button colour
- `i3quitdialog.HaltButton.color`: halt button colour
- `i3quitdialog.RebootButton.color`: halt button colour

Acceptable values:

- Hexadecimal RGB (`#0000ff`)
- One of the following strings:
    - `Black`
    - `Blue`
    - `DarkBlue`
    - `Cyan`
    - `DarkCyan`
    - `Green`
    - `DarkGreen`
    - `Magenta`
    - `DarkMagenta`
    - `Red`
    - `DarkRed`
    - `White`
    - `Yellow`
    - `DarkYellow`

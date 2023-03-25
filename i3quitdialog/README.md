[blocklet]: https://vivien.github.io/i3blocks/#_i3blocks_properties
[i3wm]: https://i3wm.org/

# I3 Quit Dialog

A simple quit dialog for [i3wm][].

## Build and install

```sh
cargo install i3quitdialog
```

## Settings

The `exit` button works out of the box. In order to allow users to halt and
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

I3 Quit Dialog supports the following keys from `xrdb`:

- `I3QuitDialog.foreground`: window colour
- `I3QuitDialog.background`: window background colour
- `I3QuitDialog.exitColor`: exit button colour
- `I3QuitDialog.cancelColor`: cancel button colour
- `I3QuitDialog.haltColor`: halt button colour
- `I3QuitDialog.rebootColor`: halt button colour

Acceptable values:

- Hexadecimal RGB (`#0000ff`)
- Hexadecimal RGBA (`#0000ffff`)
- One of the following strings:
    - `Black`
    - `Blue`
    - `Cyan`
    - `Dark1`
    - `Dark2`
    - `Dark3`
    - `DarkBlue`
    - `DarkCyan`
    - `DarkGreen`
    - `DarkMagenta`
    - `DarkRed`
    - `DarkYellow`
    - `Green`
    - `Light1`
    - `Light2`
    - `Light3`
    - `Magenta`
    - `Red`
    - `White`
    - `Yellow`

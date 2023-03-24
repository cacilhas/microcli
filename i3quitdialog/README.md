[i3wm]: https://i3wm.org/

# `i3quitdialog`

A simple quit dialog for [i3wm][].

## Build and install

```sh
cargo install i3quitdialog
```

## Settings

The `exit` button works out of the box. In order to allow the user to halt and
reboot the system, add the user to the `power` group.

You can just call it:

```sh
i3quitdialog
```

However, if you are settings a blocklet to launch `i3quitdialog`, prefer using
`i3-msg`:

```ini
[Logoff]
command=i3-msg -q -- exec i3quitdialog
color=#ff0000
full_text=⏻
text=⏻
interval=0
#signal=10  # to capture `pkill -SIGRTMIN+10 i3blocks`
```

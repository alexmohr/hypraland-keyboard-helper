# Hypraland key board helper

Simple helper program to get and change keyboard layouts


## Usage 
```
Helper program to control hypraland keyboard stuff.

Usage: hypraland-keyboard-helper [OPTIONS]

Options:
  -c, --config-file <CONFIG_FILE>      Config file path
  -p, --print-layouts <PRINT_LAYOUTS>  Print the current keyboard layout for all keyboards [possible values: true, false]
  -s, --switch-layout <SWITCH_LAYOUT>  Switch to the next layout [possible values: true, false]
  -m, --map <MAP>                      Map layouts to short string i.e. "English (US)=us;German=de"
  -l, --listen <LISTEN>                Listen for keyboard changed events and print them once they change [possible values: true, false]
  -h, --help                           Print help
```

## Waybar config

Can be used to display the keyboard layout in waybar

```
    "custom/keyboard-layout": {
      "exec": "hypraland-keyboard-helper -l true -p true -m \"English (US)=en;German=de\"",
      "format": " {}", // Icon: keyboard
      "tooltip": false,
      "on-click": "hypraland-keyboard-helper -s true",
      "on-scroll-up": "hhypraland-keyboard-helper -s true",
      "on-scroll-down": "hypraland-keyboard-helper -s true",
    },
```


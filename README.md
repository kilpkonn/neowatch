# its8020-2021

![neowatch](https://user-images.githubusercontent.com/19900308/139091622-3e3fa37b-0bc0-41f4-a7c5-75df4f981fac.gif)

# Usage
```
USAGE:
    neowatch [OPTIONS] [SUBCOMMAND]

OPTIONS:
        --change-color <COLOR>    Color for changed text [word|ANSI|rr,gg,bb]
    -d, --differences             Highlight differences since last update
    -e, --errexit                 Exit on non-zero return code
    -g, --chgexit                 Exit on output change
    -h, --help                    Print help information
    -n, --interval <SECS>         Set update insterval [default: 1.0]
        --new-color <COLOR>       Color for new text [word|ANSI|rr,gg,bb]
    -p, --precise                 Attempt to run command at precise intervals
    -V, --version                 Print version information
```

## Example
```bash
neowatch -n 0.1 --new-color=magenta --change-color=255,155,0 -d sensors

```

# Installation
Currently the `neowatch` app is available on [crates.io](https://crates.io/crates/neowatch)
```bash
cargo install neowatch
```

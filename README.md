# neowatch

[![crates.io](https://img.shields.io/badge/crates.io-v0.2.1-green)](https://crates.io/crates/neowatch)

![neowatch](https://user-images.githubusercontent.com/19900308/139555685-2d683646-1745-4cd5-a299-dcf5219108c0.gif)


# Usage
```
USAGE:
    neowatch [OPTIONS] [SUBCOMMAND]

OPTIONS:
        --change-color <COLOR>      Color for changed text [word|ANSI|rr,gg,bb]
    -d, --differences               Highlight differences since last update
        --decrease-color <COLOR>    Color for decreasing numeric values
    -e, --errexit                   Exit on non-zero return code
    -g, --chgexit                   Exit on output change
    -h, --help                      Print help information
        --increase-color <COLOR>    Color for increaseing numeric values
    -n, --interval <SECS>           Set update insterval [default: 1.0]
        --new-color <COLOR>         Color for new text [word|ANSI|rr,gg,bb]
    -p, --precise                   Attempt to run command at precise intervals
        --radix <radix>             Radix for numbers [default: 10]
    -V, --version                   Print version information
    -z, --number-changes            Highlight number changes based on increase/decrease
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

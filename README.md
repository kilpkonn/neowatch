# neowatch

[![crates.io](https://img.shields.io/badge/crates.io-v0.3.0-green)](https://crates.io/crates/neowatch)

![neowatch](https://user-images.githubusercontent.com/19900308/139555685-2d683646-1745-4cd5-a299-dcf5219108c0.gif)


# Usage
```
``
Usage:

Options:
  -n, --interval <SECS>         Set update insterval [default: 1.0]
  -d, --differences             Highlight differences since last update
  -p, --precise                 Attempt to run command at precise intervals
  -e, --errexit                 Exit on non-zero return code
  -g, --chgexit                 Exit on output change
      --new-color <COLOR>       Color for new text [word|ANSI|rr,gg,bb]
      --change-color <COLOR>    Color for changed text [word|ANSI|rr,gg,bb]
  -z, --number-changes          Highlight number changes based on increase/decrease
      --radix <radix>           Radix for numbers [default: 10]
      --increase-color <COLOR>  Color for increasing numeric values
      --decrease-color <COLOR>  Color for decreasing numeric values
  -x, --exec <SHELL>            Pass command to exec instead of `sh -c` [default: "sh -c"]
  -h, --help                    Print help
  -V, --version                 Print version`

## Example
```bash
neowatch -n 0.1 --new-color=magenta --change-color=255,155,0 -d sensors

```

# Installation
Currently the `neowatch` app is available on [crates.io](https://crates.io/crates/neowatch)
```bash
cargo install neowatch
```

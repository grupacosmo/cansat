# CanSat
Bare-metal software for the sounding rocket payload.

[Getting Started](https://grupacosmo.github.io/cansat/getting-started/index.html)

[API Reference](https://grupacosmo.github.io/cansat/api/cansat_stm32f4/index.html)

## Prerequisites
* [cargo-embed](https://github.com/probe-rs/cargo-embed) - Requires `libusb`. See `cargo-embed`'s `README.md` for instructions.
* `thumbv7em-none-eabihf` platform target
```
cargo install cargo-embed
rustup target add thumbv7em-none-eabihf
```

## xtask
[cargo-xtask](https://github.com/matklad/cargo-xtask) is a way of extending `cargo` with user-defined workflows. 
```
cargo xtask --help
```

## Build
```
cargo xtask build
```

## Run
```bash
# Runs default (crates/cansat-stm32f4) crate
# The default can be overriden with XTASK_EMBED_DEFAUL env variable
cargo xtask embed

# You can also specify the crate to run manually
cargo xtask embed -p crates/cansat-stm32f4
```

## Log filters
You can specify log levels using `DEFMT_LOG` environment variable.

Bash
```
DEFMT_LOG=debug cargo xtask embed crates/cansat-stm32f446
```

Powershell
```
$env:DEFMT_LOG=debug; cargo xtask embed crates/cansat-stm32f446
```
See https://defmt.ferrous-systems.com/filtering.html for details.

# CanSat
Software for the sounding rocket payload.

## Prerequisites
* [cargo-embed](https://github.com/probe-rs/cargo-embed) - Requires `libusb`. See `cargo-embed`'s `README.md` for instructions.
* `thumbv7em-none-eabihf` platform target
```
cargo install cargo-embed
rustup target add thumbv7em-none-eabihf
```

## Run
```bash
cargo xtask embed cansat-stm32f446
```

## Log filters
You can specify log levels using `DEFMT_LOG` environment variable.

Bash
```
DEFMT_LOG=debug cargo xtask embed cansat-stm32f446
```

Powershell
```
$env:DEFMT_LOG=debug; cargo xtask embed cansat-stm32f446
```
See https://defmt.ferrous-systems.com/filtering.html for details.

# CanSat
Software for the sounding rocket payload.

## Prerequisites
* [cargo-make](https://github.com/sagiegurari/cargo-make) - Requires `libusb`, see `cargo-make`'s `README.md` for instructions.
* [cargo-embed](https://github.com/probe-rs/cargo-embed)
* `thumbv7em-none-eabihf` platform target
```
cargo install cargo-make cargo-embed
rustup target add thumbv7em-none-eabihf
```

## Run
```
cargo make embed cansat-stm32f446
```

## Log filters
You can specify log levels using `DEFMT_LOG` environment variable.
```
cargo make --env DEFMT_LOG=debug embed cansat-stm32f446
```
See https://defmt.ferrous-systems.com/filtering.html for details.

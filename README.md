# CanSat
Software for the sounding rocket payload.

## Prerequisites
* [cargo-make](https://github.com/sagiegurari/cargo-make)
* [cargo-embed](https://github.com/probe-rs/cargo-embed) - Requires `libusb`, see `cargo-embed`'s `README.md` for instructions.
* `thumbv7em-none-eabihf` platform target
```
cargo install cargo-make cargo-embed
rustup target add thumbv7em-none-eabihf
```

## Run
```bash
cargo make embed cansat-stm32f446
```

## Log filters
You can specify log levels using `DEFMT_LOG` environment variable.
```
cargo make --env DEFMT_LOG=debug embed cansat-stm32f446
```
See https://defmt.ferrous-systems.com/filtering.html for details.

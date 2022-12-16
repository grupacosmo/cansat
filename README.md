# CanSat
## Prerequisites
 1. You will need **libusb** <br>
 check out [cargo-embed](https://crates.io/crates/cargo-embed) for instructions
 2. Add your board target:
```
rustup target add thumbv7em-none-eabihf
```

## Setup
```
cargo install cargo-embed
```

## Run
```
cd cansat-stm32f446
cargo embed
```
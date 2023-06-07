# CanSat
Bare-metal software for the sounding rocket payload.

[Getting Started](https://grupacosmo.github.io/cansat/getting-started/index.html)

[API Reference](https://grupacosmo.github.io/cansat/api/cansat_stm32f4/index.html)

## Prerequisites
* libusb:

    See [installation instructions](https://github.com/probe-rs/probe-rs/tree/master/cargo-embed#prerequisites).

* [cargo-embed](https://github.com/probe-rs/cargo-embed):

    ```bash
    cargo install cargo-embed
    ```

## xtask
[cargo-xtask](https://github.com/matklad/cargo-xtask) is a way of extending `cargo` with user-defined workflows.

Run the following commands for a quick rundown:
```
cargo xtask --help

cargo xtask embed --help
```

All workflows are implemented in the `xtask/` directory.

## Build
```
cargo xtask build
```

## Run
```bash
# Runs the default binary crate (cansat-stm32f4)
# The default can be overriden with XTASK_EMBED_DEFAULT env variable in .cargo/config.toml
cargo xtask embed

# You can also specify the crate to run manually
cargo xtask embed -p cansat-stm32f4
```

## Log filters
You can specify log levels using `DEFMT_LOG` environment variable.

Bash
```bash
DEFMT_LOG=debug cargo xtask embed
```

Powershell
```powershell
$env:DEFMT_LOG="debug"; cargo xtask embed
```
See https://defmt.ferrous-systems.com/filtering.html for details.

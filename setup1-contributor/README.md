# setup1-contributor

## Build Guide

To build the contributor run:

```bash
# Requires specific rustc version
rustup default 1.48

# Build the binary
cargo build --release
```

To add the binary to the PATH run: T

```bash
cargo install --path .
```

This will add the `aleo-setup-contributor` binary to `.cargo/bin` folder

## Usage

Generate a seed and a private key:

```bash
aleo-setup-contributor generate --keys-path keys.json
```

Run the contributor:
```bash
aleo-setup-contributor --api-url https://... --keys-path keys.json
```
where `--api-url` is a coordinator api address
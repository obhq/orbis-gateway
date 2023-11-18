# Orbis Gateway

Orbis Gateway is an application running inside the PS4 kernel to provides the interfaces to the PS4 internal.

## Building from source

### Prerequisites

- Rust on the latest stable channel

### Enable x86_64-unknown-none target

```sh
rustup target add x86_64-unknown-none
```

### Install additional Cargo commands

```sh
cargo install cargo-binutils
```

`cargo-binutils` required additional dependency which can be installed with the following command:

```sh
rustup component add llvm-tools-preview
```

## License

MIT

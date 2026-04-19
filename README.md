# scimantic

Command-line companion for the Scimantic scientific knowledge management platform.

## Installation

```bash
cargo install scimantic
```

## Usage

```bash
scimantic --version
scimantic --help
```

## Development

See [CLAUDE.md](CLAUDE.md) for development commands.

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (MSRV: 1.88)
- [prek](https://github.com/j178/prek) for pre-commit hooks: `cargo install prek && prek install`

### Build and test

```bash
cargo build
cargo nextest run
```

## License

MIT License. See [LICENSE](LICENSE).

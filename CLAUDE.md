# scimantic

Command-line companion for the Scimantic scientific knowledge management platform.

## Development

```bash
cargo build              # build
cargo nextest run        # run tests
cargo test --doc         # doc tests
cargo clippy             # lint
cargo fmt                # format
cargo audit              # security scan
cargo deny check         # license/dependency check
cargo vet                # supply chain review
```

Mutation testing (cargo-mutants, scoped via `.cargo/mutants.toml`):

```bash
cargo install cargo-mutants   # once per machine
./scripts/mutants.sh          # mutate changes in HEAD~1..HEAD (fast, --in-diff)
./scripts/mutants.sh main     # mutate everything changed since main
cargo mutants                 # full sweep of all in-scope code (slow)
```

CI runs the fast `--in-diff` variant on every push/PR and a full sweep
weekly; both fail on a surviving mutant.

## Pre-commit hooks

```bash
cargo install prek
prek install
```

Hooks mirror CI checks: fmt, clippy, check, nextest, doctest, audit, deny, vet.

## Release process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Commit: `git commit -m "Release vX.Y.Z"`
4. Tag: `git tag vX.Y.Z`
5. Push: `git push origin main --tags`

The tag triggers CI which builds, tests, creates a GitHub Release, and publishes to crates.io.

<!-- Add custom skills under .claude/skills/ as needed -->

# cactus_rs

![GitHub](https://img.shields.io/github/license/Cactus-Network/cactus_rs?logo=Github)
[![Coverage Status](https://coveralls.io/repos/github/Cactus-Network/cactus_rs/badge.svg?branch=main)](https://coveralls.io/github/Cactus-Network/cactus_rs?branch=main)
![Build Crate](https://github.com/Cactus-Network/cactus_rs/actions/workflows/build-crate.yml/badge.svg)
![Build NPM](https://github.com/Cactus-Network/cactus_rs/actions/workflows/build-npm.yml/badge.svg)
![Build Wheels](https://github.com/Cactus-Network/cactus_rs/actions/workflows/build-wheels.yml/badge.svg)

![PyPI](https://img.shields.io/pypi/v/cactus_rs?logo=pypi)
![PyPI - Format](https://img.shields.io/pypi/format/cactus_rs?logo=pypi)
[![Crates.io](https://img.shields.io/crates/v/cactus.svg)](https://crates.io/crates/cactus)
[![Downloads](https://img.shields.io/crates/d/cactus.svg)](https://crates.io/crates/cactus)
[![Docs](https://docs.rs/cactus/badge.svg)](https://docs.rs/cactus/latest/cactus/)

A collection of Rust crates for working with the Cactus blockchain. There are also Python bindings in the form of a wheel.

## Prerequisites

- [Python](https://www.python.org/downloads/) 3.9 or higher installed.
- The [Rust toolchain](https://rustup.rs/) must be installed.

## Unit Tests

To run the unit tests for the whole workspace:

```bash
cargo test --workspace
```

Some slow tests are only enabled in optimized builds, so it may also be a good idea to run the tests in release mode:

```bash
cargo test --workspace --release
```

### Python Linking

You may need a Python virtual environment activated for the tests to link properly, due to the `pyo3` dependency in `wheel`.

You can setup a virtual environment with the following command:

```bash
python3 -m venv venv
```

Activate the virtual env:

```bash
. ./venv/bin/activate
```

### Python Tests

The `wheel` crate is a single Python wheel that exports bindings to various functionality in the repository, mostly from `cactus-consensus` and `cactus-protocol`.

It's built with `maturin`, so you need to have activated a python virtual environment for the build to work.

The bindings are tested with `pytest`. Before you run them, install the following dependencies:

```bash
pip install pytest maturin typing-extensions cactus-blockchain==2.1.2
```

And build the Python wheel:

```bash
maturin develop -m wheel/Cargo.toml
```

Finally, you can run the Python binding tests:

```bash
pytest tests
```

Note that these tests can take several minutes to complete.

## Benchmarks

To run benchmarks for a specific crate before you make changes:

```bash
cargo bench -- --save-baseline before
```

After you apply the changes, finish the benchmark:

```bash
cargo bench -- --save-baseline after
critcmp after before
```

You can also run all the benchmarks by including `--workspace`.

Note that you must include the flag before the `--`, for example:

```bash
cargo bench --workspace -- --save-baseline before
```

## Precommit Hook

This repository has a pre-commit configuration, which is hooked into git by running:

```bash
pre-commit install --hook-type pre-commit --hook-type pre-push
```

It runs Prettier and then `cargo fmt` on all crates on every commit. When you push, it runs `cargo clippy`, `cargo test`, and `cargo build`.

To run all checks explicitly (without pushing), run:

```bash
pre-commit run --all --hook-stage pre-push
```

## Fuzzers

Fuzzers can't be run or listed for the whole workspace, but only for individual crates. There is a tool to generate a fuzzing corpus from a blockchain database.

It's run like this:

```bash
cd crates/cactus-tools
cargo run --release --bin gen-corpus -- --help
```

The following crates have fuzzers:

- cactus-bls
- cactus-consensus
- cactus-protocol
- cactus-puzzles
- clvm-utils

To list and run fuzzers:

```bash
cargo fuzz list
cargo +nightly fuzz run <name-of-fuzzer> --jobs=10
```

## Bumping Version Number

Make sure you have `cargo-workspaces` installed:

```bash
cargo install cargo-workspaces
```

To bump the versions of all relevant crates:

```bash
cargo ws version --all --no-git-commit
```

Select "minor update" if there has not been any incompatible API changes, otherwise "major update".

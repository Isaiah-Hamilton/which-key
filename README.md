<div align="center">
  <h1>Which Key</h1>

  <p>helps you remember your keymaps without leaving the terminal. Inspired by [which-key.nvim](https://github.com/folke/which-key.nvim)</p>

  [<img src="https://img.shields.io/github/v/release/isaiah-hamilton/which-key" alt="github version">](https://github.com/Isaiah-Hamilton/which-key/releases)
</div>

## Installation

### Cargo

Installation via `cargo` can be done by installing the [`which-key`](https://crates.io/crates/which-key) crate:

```bash
cargo install which-key
```

Alternatively, if you can use `cargo install` using the repo as the source.

```bash
# Option 1 - Download from releases and install
curl -LO https://github.com/isaiah-hamilton/which-key/archive/0.10.2.tar.gz
tar -xzvf 0.10.2.tar.gz
cargo install --path . --locked

# Option 2 - Manually clone the repo and install
git clone https://github.com/isaiah-hamilton/which-key
cd bottom
cargo install --path . --locked

# Option 3 - Install using cargo with the repo as the source
cargo install --git https://github.com/ClementTsang/bottom --locked
```

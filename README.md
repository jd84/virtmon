# virtmon

virtmon is yet another monitoring tool.

## Features

virtmon supports some typical features:

* CPU, memory, and network usage visualization

It also aims to be:

* Fast and lightwight
* Cross-platform - supports Linux, Window, macOS and RaspberryPi

## Installation

virtmon is build on the stable version of Rust `1.44.0`.

### Manual

```
# If required, update Rust on the stable channel
rustup update stable

# Clone and install the newest master version all via Cargo
cargo install --git https://github.com/jd84/virtmon

# Clone from master and install manually
git clone https://github.com/jd84/virtmon.git
cd bottom
cargo install --path .

# Download from releases and install
curl -LO https://github.com/jd84/virtmon/releases/download/0.1.0/virtmon_source_code.tar.gz
tar -xzvf virtmon_source_code.tar.gz
cargo install --path .
```

### Cargo

```
cargo install virtmon
```
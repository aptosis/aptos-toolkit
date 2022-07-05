# Aptos Toolkit 🛠

A collection of scripts and tools for interacting with the [Aptos blockchain](https://aptoslabs.com).

_Documentation is currently extremely sparse but will be improved in the near future._

## Setup

Install the CLI using Cargo:

```bash
cargo install move-idl-parse

# On Sui
cargo install move-idl-parse --features address20

# On Aptos
cargo install move-idl-parse --features address32
```

## Usage

In a directory containing a `Move.toml`, run:

```
move-idl-parse
```

This will generate a series of JSON files in your `build/idls/` directory.

## License

Aptos Toolkit is licensed under the Apache License, Version 2.0.
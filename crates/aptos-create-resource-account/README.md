# aptos-create-resource-account

CLI for creating Aptos resource accounts.

*Note: this CLI will be changed in the near future, since resource accounts are still being developed in aptos-core.*

## Setup

To use this CLI, you may install this crate via Cargo:

```bash
cargo install aptos-create-resource-account
```

You should also have an Aptos CLI configuration file, which can be created using:

```bash
aptos init
```

## Usage

Run the following command to create a new resource account:

```bash
aptos-create-resource-account my-account
```

This will create a profile in your `.aptos/config.yaml` with the name `my-account`.
You can then use this profile to publish modules, run scripts, etc.

License: Apache-2.0

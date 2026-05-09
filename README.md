# SoroStandards

> The ERC-20/ERC-721 moment for Soroban — a production-ready, auditable library of contract standards any developer can fork and deploy in minutes.

[![CI](https://github.com/Stellar-Project-Hub/SoroStandards/actions/workflows/ci.yml/badge.svg)](https://github.com/Stellar-Project-Hub/SoroStandards/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## Standards Included

| Contract | Path | Status |
|---|---|---|
| SEP-41 Fungible Token | `contracts/sep41-fungible` | 🚧 In Progress |
| NFT | `contracts/nft` | 🚧 In Progress |
| Multisig | `contracts/multisig` | 🚧 In Progress |
| Vesting | `contracts/vesting` | 🚧 In Progress |
| Timelocks | `contracts/timelocks` | 🚧 In Progress |

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup): `cargo install --locked soroban-cli`
- Node.js ≥ 18 (for TypeScript tests)

## Local Development

```bash
# Clone
git clone https://github.com/Stellar-Project-Hub/SoroStandards.git
cd SoroStandards

# Build all contracts
cargo build --release --target wasm32-unknown-unknown

# Run all Rust unit tests
cargo test

# Run TypeScript integration tests
cd tests/ts && npm ci && npm test
```

## Repository Structure

```
SoroStandards/
├── contracts/
│   ├── sep41-fungible/   # SEP-41 fungible token standard
│   ├── nft/              # Non-fungible token standard
│   ├── multisig/         # M-of-N multisig wallet
│   ├── vesting/          # Linear vesting with cliff
│   └── timelocks/        # Time-locked token release
├── tests/ts/             # TypeScript integration tests
├── scripts/              # Deployment & utility scripts
├── docs/                 # Specification documents
├── .github/workflows/    # CI/CD pipelines
└── Cargo.toml            # Workspace root
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT

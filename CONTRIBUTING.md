# Contributing to SoroStandards

Thank you for contributing! This guide is designed for async, parallel work — you do **not** need to understand the entire codebase to make a meaningful contribution.

## Quick Start

```bash
git clone https://github.com/YOUR_ORG/SoroStandards.git
cd SoroStandards
rustup target add wasm32-unknown-unknown
cargo build
cargo test
```

## Directory Map — Where to Work

Each contract is a fully self-contained Rust crate. **You only need to touch the directory for your issue.**

```
contracts/
├── sep41-fungible/src/lib.rs   ← SEP-41 fungible token logic
├── nft/src/lib.rs              ← NFT mint/transfer/burn logic
├── multisig/src/lib.rs         ← M-of-N multisig execution
├── vesting/src/lib.rs          ← Linear vesting with cliff
└── timelocks/src/lib.rs        ← Time-locked token release

tests/ts/                       ← TypeScript integration tests (Node ≥ 18)
docs/                           ← Specification markdown files
scripts/                        ← Deployment helper scripts (bash/JS)
```

**Safe zones for contributors** — you will never need to modify:
- `Cargo.toml` (workspace root) — maintainer-only
- `.github/workflows/` — maintainer-only
- Another contract's directory unless your issue explicitly spans multiple contracts

## Claiming an Issue

1. Find an open issue labelled `drips-wave`.
2. Comment `I'd like to work on this` — a maintainer will assign it within 24 h.
3. Each issue has a **7-day sprint window**. If you need more time, comment on the issue.

## Branch Naming

```
<type>/<issue-number>-<short-slug>
# Examples:
feat/4-sep41-transfer
fix/3-nft-token-uri
docs/1-contributing-guide
```

## Pull Request Checklist

Before opening a PR, confirm:

- [ ] `cargo fmt --all` produces no diff
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo test --all` passes
- [ ] New public functions have doc comments (`///`)
- [ ] If you added a new entry-point, update the relevant `docs/` spec file
- [ ] CI is green on your branch

## Commit Style

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(sep41): implement transfer and transfer_from
fix(nft): correct owner lookup after burn
docs(vesting): add cliff calculation example
test(multisig): add threshold boundary tests
```

## Code Style

- `cargo fmt` is enforced by CI — run it before every commit.
- `cargo clippy -- -D warnings` is enforced by CI.
- No `unwrap()` in contract code — use explicit panics with messages.
- All storage keys must be defined in a `DataKey` enum.

## Questions

Open a Discussion or ping `@maintainers` in your issue thread.

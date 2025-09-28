# Holographic Brick Chain

**A tiny, deterministic, proof-ready token ledger with a "hologram" super-root, constant attestations, and an embedded-first design.**  
Inspired by ideas around resonance and constant oscillation (after Itzhak Bentov).

## What this is
- Clean Rust workspace split into small crates (`brick-*`)
- Deterministic ledger (AccountId-based), canonical bytes, BLAKE3
- Merkle proofs for balances
- Super-root ("hologram") over zone commits
- Real Ed25519 attestations every tick
- Headless CLI to exercise all flows

## Quick start
```bash
cargo build --workspace
cargo run -p brick-cli -- hello
cargo run -p brick-cli -- holo_tick
cargo run -p brick-cli -- holo_run 5
cargo run -p brick-cli -- prove alice


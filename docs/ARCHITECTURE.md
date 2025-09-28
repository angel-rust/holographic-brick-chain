# Architecture

Holographic Brick Chain = many small crates with strict roles:

- `brick-core`: ledger (Mint/Transfer), deterministic, AccountId-based
- `brick-codec`: canonical bytes (TXv3)
- `brick-chain`: roots & Merkle proofs
- `brick-lanes`: conflict-free packing for parallelism
- `brick-exec`: lane execution (sequential baseline)
- `brick-da`: DA commitment (minimal for now)
- `brick-super`: super-root over zone commits ("hologram")
- `brick-attest`: signed, chained attestations (Ed25519)
- `brick-holo`: orchestrator (tick → seal → claim → super-root → attest)
- `apps/brick-cli`: headless driver for demos/bench

Determinism pillars:
- IDs are numbers (AccountId from normalized names)
- Canonical encodings with version tags
- Sorted order before hashing
- Same inputs => same outputs on any machine

# Canonical Bytes

- **TXv3** (`BRICK/TXv3`): LE numbers, memo length ≤ 128, writes sorted+deduped.
- **LEAF v1** (`BRICK/LEAF/v1`): (acct,u64, bal,u128)
- **ATTEST v1** (`BRICK/ATTEST/v1`): header fields in strict order
- **SUPER v1** (`BRICK/SUPER/v1`): (zone, height, state_root, da_root) per zone
- **DA v1** (`BRICK/DAv1`): domain tag + data hash

All lists sorted before hashing. All integers little-endian.

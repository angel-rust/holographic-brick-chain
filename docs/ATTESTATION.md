# Attestation

Every tick emits a signed header:

{ node_id, epoch, zone, height, tx_root, state_root, da_root, super_root?, prev? }

- Hash: blake3 of the canonical bytes ("BRICK/ATTEST/v1" ... )
- Signature: Ed25519 over the header bytes
- Chain: `prev` must match last attestation's hash

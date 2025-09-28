# Limits

- Memo length: ≤ 128 bytes (truncate)
- batch_max: default 8192 (configurable in `brick-holo` config)
- Writes: sorted + deduped (by AccountId)
- No self-transfer (explicitly error)

# Module 8: OutpaceInflation (OIM)

## Purpose
OutpaceInflation (OIM) is a policy module that governs treasury/savings positioning to target positive real return over time (i.e., outpace inflation), without modifying token supply or violating LAW invariants.

OIM does NOT:
- mint, inflate, or modify supply
- change deterministic unlock windows / timelock schedules
- promise yield or run discretionary strategies
- rely on price feeds for market timing

OIM DOES:
- define an inflation hurdle (oracle-fed or fixed at genesis)
- compute a real-return score from deterministic accounting
- adjust treasury allocation between Safety and Growth buckets within hard caps
- fail-safe to freeze when inflation data is stale (oracle mode)

## Modes

### 1) Oracle Mode
- Authorized oracle posts inflation (bps) per period
- OIM enforces freshness (staleness => freeze)

### 2) Fixed Hurdle Mode
- No oracle required
- Fixed annual inflation hurdle (bps) set at genesis and immutable

## Deterministic Buckets

Treasury/savings constrained to:

- Safety bucket (stables / low-vol)
- Growth bucket (strategies, capped)
- Liquidity bucket (LP/support, capped)

Burn/Airdrop buckets remain governed by existing modules.

OIM may only rebalance Safety <-> Growth within caps and cooldown.

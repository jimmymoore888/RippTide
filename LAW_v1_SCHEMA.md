# LAW v1 — CANONICAL SCHEMA

Status: Immutable once deployed  
Protocol: NEAR  
Deployment Mode: Irreversible

---

## SECTION 0 — META

law_version: 1  
protocol: NEAR  
authoritative_schema: true  

---

## SECTION 1 — INTENT

intent:
  acknowledgment_required: true
  text_hash: SHA256("This system creates irreversible economic law.")

---

## SECTION 2 — IDENTITY

identity:
  asset_name: string (1–32 chars)
  ticker: string (2–6 chars, A–Z only)
  network: NEAR

---

## SECTION 3 — SUPPLY LAW

supply:
  mode: FIXED | CAPPED | MINTABLE

  fixed:
    total_supply: u128 > 0

  capped:
    max_supply: u128 > 0
    initial_mint: u128 <= max_supply

  mintable:
    governance_required: true

---

## SECTION 4 — SAFETY SYSTEMS

### Burn

burn:
  mode: NONE | MANUAL | AUTOMATED
  automated:
    burn_cap_percent: 0–100
    pause_allowed: false

### Time Lock

time_lock:
  enabled: true | false
  deterministic:
    cadence: 2Y
    anchor_date: NOV-22
    claim_window_days: 7

---

## SECTION 5 — DISTRIBUTION

distribution:
  liquidity_pool_percent: u8
  airdrop_percent: u8
  savings_percent: u8
  total: 100

---

## SECTION 6 — LAW TEXT (AUTO-GENERATED)

law_text:
  generated_from: LAW_v1_SCHEMA
  immutable: true

---

## SECTION 7 — FINAL CERTIFICATION

certification:
  scroll_required: true
  certify_toggle_required: true
  hold_to_confirm_seconds: 3

---

## SECTION 8 — COMPILATION OUTPUT

compile:
  input: LAW_v1_SCHEMA
  output:
    - contract.wasm
    - law_text_hash
    - invariant_hash

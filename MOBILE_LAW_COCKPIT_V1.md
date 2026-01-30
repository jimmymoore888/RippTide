# MOBILE LAW COCKPIT — V1
Render Specification (Read-Only Logic)

Target Devices:
- iPhone 10+
- Android equivalent
- Safari / WebView compatible

Authoritative Source:
- LAW_v1_SCHEMA.md

---

## PANEL ORDER (FIXED)

0. Intent Certification  
1. Asset Identity  
2. Supply Law  
3. Safety Systems  
4. Distribution  
5. Law Review  
6. Final Authorization  

Panels MUST render sequentially.
No skipping allowed.

---

## PANEL 0 — INTENT CERTIFICATION

Source:
- SECTION 1 — INTENT

UI Elements:
- Read-only warning text
- Checkbox: acknowledgment_required
- Button: BEGIN LAW (disabled until checked)

Rules:
- No navigation forward without acknowledgment
- Text is hash-locked

---

## PANEL 1 — ASSET IDENTITY

Source:
- SECTION 2 — IDENTITY

UI Elements:
- Text field: asset_name
- Text field: ticker
- Read-only label: NEAR Protocol

Validation:
- Length and charset enforced locally
- Collision warnings allowed (non-blocking)

---

## PANEL 2 — SUPPLY LAW

Source:
- SECTION 3 — SUPPLY LAW

UI Elements:
- Radio selector: supply.mode
- Sliders:
  - total_supply (FIXED)
  - max_supply / initial_mint (CAPPED)

Rules:
- Exactly one mode selectable
- Invalid combinations impossible
- MINTABLE flagged ADVANCED

---

## PANEL 3 — SAFETY SYSTEMS

### Burn

Source:
- SECTION 4A — BURN

UI Elements:
- Toggle group: burn.mode
- Slider: burn_cap_percent (AUTOMATED only)

Rules:
- Pause not exposed
- Burn is irreversible (label only)

---

### Time Lock

Source:
- SECTION 4B — TIME LOCK

UI Elements:
- Toggle: enabled
- Fixed schedule selector (read-only):
  - Every 2 years
  - Nov 22
  - 7-day window

Rules:
- No date input
- No overrides

---

## PANEL 4 — DISTRIBUTION

Source:
- SECTION 5 — DISTRIBUTION

UI Elements:
- Sliders:
  - liquidity_pool_percent
  - airdrop_percent
  - savings_percent

Visual:
- Fuel-bar style allocation graph

Rules:
- Sliders auto-balance to 100%
- No text input

---

## PANEL 5 — LAW REVIEW

Source:
- SECTION 6 — LAW TEXT

UI Elements:
- Read-only generated law text
- Status indicators:
  - Green = valid
  - Yellow = warning
  - Red = blocked

Rules:
- Non-editable
- Rendered directly from schema output

---

## PANEL 6 — FINAL AUTHORIZATION

Source:
- SECTION 7 — FINAL CERTIFICATION

UI Elements:
- Scroll-required law text
- Certification checkbox
- Hold-to-confirm button (3 seconds)

Rules:
- Hold duration enforced
- Single-execution only

---

## POST-DEPLOY MODE — LOGBOOK

UI State:
- Read-only

Displays:
- Deployed law text
- Transaction history
- Deterministic timeline

Edit controls MUST NOT render.

---

## UI AXIOM

If a control is not defined here,
it MUST NOT exist in the UI.

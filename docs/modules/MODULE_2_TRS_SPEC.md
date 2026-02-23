# Module 2 Extension Spec (TRS v0)
## Truck-Industry Stabilization Primitives (On-Chain Ready)

**Canonical Name:** TRS — Trucking Resilience Submodule  
**Host Module:** Module 2 — Treasury Allocation State Machine  
**Doctrine Dependencies:** Module 8 (OIM) mandatory + permanent (system doctrine)  
**Primary Objective:** Reduce trucking company closures driven by inflation + rate compression by providing **non-inflationary**, **rule-bound**, **receivable-backed working capital** and **cost-shock buffering**.

---

## 1) Economic Thesis (Why TRS Exists)
Trucking failures under inflation typically follow this sequence:
1) Freight rates/volume soften (revenue compresses),
2) Operating costs remain elevated (insurance, maintenance, labor, fuel proxy),
3) Cash conversion cycles break (slow-paying invoices),
4) High-cost factoring/credit accelerates insolvency,
5) Restructuring / Chapter 11.

**TRS targets (3) and (4)** with deterministic liquidity primitives funded from treasury (not minting).

---

## 2) Design Principles (Hard Constraints)
**DP-01 Non-Inflationary:** TRS MUST NOT mint. All support is funded from treasury balances.  
**DP-02 Deterministic Eligibility:** No discretionary committee approvals; rule-based + auditable.  
**DP-03 Short-Duration Liquidity:** Bridge capital only; strict tenors + caps.  
**DP-04 Adversarial Safety:** Protect against invoice fraud, sybil fleets, oracle manipulation, “zombie carrier” dependency.  
**DP-05 Solvency First:** Reserve floors and exposure caps always enforced.

---

## 3) Treasury Partitioning (Module 2 Addendum)
Module 2 MUST create/maintain these TRS buckets:

### 3.1 Buckets
- **LT (Liquidity Treasury):** funds advances to carriers
- **RF (Reserve Floor):** untouchable minimum reserve for systemic solvency
- **LF (Loss Fund):** capped loss absorber for verified defaults
- **AF (Admin/Fee Sink):** collects fees; can route to OIM-directed savings bias if doctrine requires

### 3.2 Global Invariants
- **INV-TRS-01 Reserve Floor:** `LT_balance_after >= RF_min`
- **INV-TRS-02 Max Exposure:** `outstanding_total <= LT_balance * EXPOSURE_CAP`
- **INV-TRS-03 Per-Carrier Cap:** `outstanding_carrier <= CARRIER_CAP`
- **INV-TRS-04 No Mint:** any `advance_amount` must be debited from LT balance
- **INV-TRS-05 Loss Fund Cap:** `LF_balance <= LF_CAP * LT_inflows_total`
- **INV-TRS-06 Concentration:** `outstanding_by_payer <= PAYER_CAP` (optional but recommended)

---

## 4) Actors
- **Carrier:** trucking company / owner-operator (participant)
- **Payer:** shipper/broker owing invoices (optional in v0 if no direct payer integration)
- **Attestor:** verifies invoices / proof-of-delivery (oracle/third party)
- **Oracle:** publishes stress and index signals used for term adjustments
- **Treasury Router:** Module 2 logic controlling LT/RF/LF/AF flows

---

## 5) State Machines

### 5.1 Carrier State
- `UNREGISTERED`
- `REGISTERED`
- `ACTIVE`
- `SUSPENDED` (risk/fraud/delinquency)
- `EJECTED` (hard ban)

**Transitions**
- `register_carrier`: UNREGISTERED → REGISTERED  
- `activate`: REGISTERED → ACTIVE  
- `suspend_carrier`: ACTIVE/REGISTERED → SUSPENDED  
- `reinstate_carrier`: SUSPENDED → ACTIVE  
- `eject_carrier`: any → EJECTED

### 5.2 Receivable State
- `SUBMITTED`
- `VERIFIED`
- `ADVANCED`
- `REPAID`
- `DEFAULTED`
- `REJECTED`

**Transitions**
- `submit_receivable`: (none) → SUBMITTED  
- `verify_receivable`: SUBMITTED → VERIFIED  
- `reject_receivable`: SUBMITTED → REJECTED  
- `execute_advance`: VERIFIED → ADVANCED  
- `repay`: ADVANCED → REPAID  
- `mark_default`: ADVANCED → DEFAULTED

---

## 6) Core Primitive: Receivable-Backed Liquidity (RBL)
TRS provides carriers an advance against verified receivables, replacing expensive off-chain factoring.

### 6.1 Verification Tiers (Risk Bounds)
- **Tier A (Strong):** payer acknowledgement or strong attestation (highest advance, lowest fee)
- **Tier B (Medium):** attestation + carrier history (moderate terms)
- **Tier C (Weak/Optional):** limited verification (low terms or disabled)

**Rule:** weaker verification ⇒ lower advance %, higher fee, tighter caps.

---

## 7) Cost Shock Buffer (CSB) via StressIndex
TRS adjusts terms as inflation/rate stress increases, within bounded rails.

### 7.1 StressIndex Definition
A scalar `StressIndex (SI) ∈ [0, 1]` published by an oracle set.

**Example weighted formula (oracle internal):**
`SI = clamp(w1*CostPressure + w2*RatePressure + w3*DistressSignal, 0, 1)`

TRS contract treats SI as an input value with:
- update cadence constraints
- authorized updater(s)
- min/max delta per epoch (anti-manipulation)

### 7.2 Term Adjustments (Bounded)
- **Advance %** increases slightly with SI (capped)
- **Fee bps** decreases slightly with SI (floored)
- **Tenor days** increases slightly with SI (capped)

**Hard bounds prevent subsidy drift.**

---

## 8) Pricing & Limits

### 8.1 Advance Percent by Tier (Defaults)
- Tier A: `ADV_PCT_A ∈ [0.80, 0.90]`
- Tier B: `ADV_PCT_B ∈ [0.60, 0.80]`
- Tier C: `ADV_PCT_C ∈ [0.30, 0.60]` (or disable)

### 8.2 Fee Schedule
Let `fee_bps = clamp(BASE_BPS - (SI * DISCOUNT_BPS), MIN_BPS, MAX_BPS)`

### 8.3 Tenor
Let `tenor_days = clamp(BASE_DAYS + (SI * EXTENSION_DAYS), MIN_DAYS, MAX_DAYS)`

### 8.4 Default + Grace
- `grace_days`: after due date before default eligibility
- `default_requires_proof`: attestor/oracle must provide evidence of non-payment

---

## 9) Interface (Functions)

### 9.1 Carrier Lifecycle
- `register_carrier(carrier_id, metadata_hash, tier) -> bool`
- `activate_carrier(carrier_id) -> bool`
- `update_carrier_tier(carrier_id, tier) -> bool` *(restricted)*
- `suspend_carrier(carrier_id, reason_code) -> bool` *(restricted)*
- `reinstate_carrier(carrier_id) -> bool` *(restricted)*
- `eject_carrier(carrier_id, reason_code) -> bool` *(restricted)*

### 9.2 Receivable Handling
- `submit_receivable(carrier_id, receivable_hash, face_value, payer_ref, due_date, verification_hint) -> receivable_id`
- `verify_receivable(receivable_id, verification_proof) -> bool` *(restricted)*
- `reject_receivable(receivable_id, reason_code) -> bool` *(restricted)*

### 9.3 Quotes + Advances
- `quote_advance(receivable_id) -> {advance_amount, fee_amount, tenor_days, repayment_ref}`
- `execute_advance(receivable_id) -> bool`
  - checks: carrier ACTIVE, receivable VERIFIED, caps/invariants, LT_balance, RF_min
  - transfers: LT → carrier (`advance_amount`)
  - marks: receivable ADVANCED
  - records: `principal`, `fee`, `tenor`, `deadline`

### 9.4 Repayment
- `repay(receivable_id, amount) -> bool`
  - routes: principal → LT
  - routes: fee → AF (and optional share → LF)
  - marks: REPAID when satisfied

- `mark_default(receivable_id, default_proof) -> bool` *(restricted)*
  - checks: deadline + grace passed
  - draws: LF up to capped coverage ratio (optional)
  - marks: DEFAULTED
  - updates: carrier risk (suspend/eject thresholds)

### 9.5 Stress Updates
- `update_stress_index(new_SI, proof) -> bool` *(restricted)*
  - checks: authorized updater, cadence, delta bounds
  - writes: SI

---

## 10) Anti-Fraud & Abuse Controls (v0)
- **AF-01 Cooldown:** min time between advances per carrier
- **AF-02 Invoice Uniqueness:** receivable_hash must be unique (prevent double-pledge)
- **AF-03 Concentration Limits:** payer_ref exposure caps
- **AF-04 Tiered Caps:** Tier C has strict caps; may be disabled entirely
- **AF-05 Suspension Triggers:** repeated late repayments / disputed proofs

---

## 11) Event Log (Audit Reconstruction)
Emit events for deterministic reconstruction:

- `EVT_TRS_CARRIER_REGISTERED(carrier_id, tier, metadata_hash)`
- `EVT_TRS_CARRIER_STATUS(carrier_id, from_state, to_state, reason_code)`
- `EVT_TRS_RECEIVABLE_SUBMITTED(receivable_id, carrier_id, face_value, payer_ref, due_date)`
- `EVT_TRS_RECEIVABLE_VERIFIED(receivable_id, verifier_id)`
- `EVT_TRS_ADVANCE_EXECUTED(receivable_id, carrier_id, advance_amount, fee_amount, tenor_days, SI)`
- `EVT_TRS_REPAYMENT(receivable_id, amount, remaining_balance)`
- `EVT_TRS_DEFAULT(receivable_id, loss_drawn, carrier_action)`
- `EVT_TRS_STRESSINDEX_UPDATED(old_SI, new_SI, updater_id)`

---

## 12) Minimal v0 Test Plan (Pass/Fail)
1. **No Mint:** `execute_advance` fails without LT funds  
2. **Reserve Floor:** any action violating `RF_min` reverts  
3. **Exposure Cap:** outstanding_total beyond cap reverts  
4. **Per-Carrier Cap:** outstanding_carrier beyond cap reverts  
5. **Tier Bound:** advance % respects tier limits  
6. **Stress Bounds:** SI updates respect cadence + delta caps  
7. **Double Pledge:** duplicate receivable_hash rejected  
8. **Default Path:** overdue + grace + proof → DEFAULTED and LF draw capped  
9. **Event Completeness:** every state transition emits the expected event

---

## 13) Near-Intersect Integration Notes
- TRS is packaged as **Module 2 policy extension**: `M2::TRS`
- It does not change RagTuff law; it instantiates deterministic economic behavior under Module 2 constraints.
- OIM remains macro-discipline; TRS is micro-liquidity survival without inflationary issuance.

---

## 14) Implementation Targets (On-Chain Readiness)
v0 implementation should ship with:
- deterministic caps + invariant checks
- oracle/attestor authorization model
- full event coverage
- unit tests for the v0 test plan above

**File Name Recommendation:** `MODULE_2_TRS_SPEC.md`

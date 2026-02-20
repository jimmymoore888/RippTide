# Near-Intersect Enforcement Matrix v0
## LAW_v1 + Module Invariants → Gates → Runtime Assertions → Audit Events

This matrix maps:
- LAW invariants and module invariants
to:
- enforcement points (compile/deploy/runtime)
- responsible modules/nodes
- reject codes
- required events

This is constitutional plumbing, not marketing.

---

# 1) Enforcement Points (Canonical)

EP-1  COMPILE_GATE    (LAW validation / matrix compilation)
EP-2  DEPLOY_GATE     (factory allow/deny + registry conformance)
EP-3  RUNTIME_ASSERT  (in-contract invariant assertions)
EP-4  OBS_AUDIT       (events for reconstruction / auditing)

Nodes (from DIAGNOSTIC_GRAPH_v0):
- N3 Registry Lookup
- N4 LAW Validation Engine
- N5 Enforcement Matrix Compiler
- N6 Deployment Gate
- N8 Factory Contract Instantiation
- N9 Runtime Invariant Layer
- N10 Treasury/Governance Execution
- N11 Observability (events)
- N12 OIM Runtime Governor (Module 8)

---

# 2) LAW_v1 Enforcement Matrix (v0)

Legend:
- MUST = hard fail / reject / revert
- SHOULD = recommended auditability / safety

| Invariant ID | Invariant (summary) | EP-1 Compile Gate | EP-2 Deploy Gate | EP-3 Runtime Assert | Primary Module(s) | Node Owner(s) | Reject Code | Required Events |
|---|---|---|---|---|---|---|---|---|
| LAW-01 | LAW schema validity (required fields/types) | MUST validate schema version + canonical serialization | MUST deny deploy if invalid | N/A | LAW_v1 | N4,N5,N6 | LAW_001_SCHEMA_INVALID | LawValidationStarted, LawValidationRejected |
| LAW-02 | LAW integrity/authority (checksum/signature if configured) | MUST verify checksum/signature + forbid downgrade | MUST deny deploy if integrity fails | SHOULD log integrity checks | LAW_v1 | N4,N6 | LAW_002_INTEGRITY_FAIL | LawValidationRejected |
| LAW-03 | Deterministic module requirements (doctrine constraints) | MUST enforce required modules + counts + stacking caps | MUST deny deploy if doctrine fails | N/A | Factory + Doctrine | N4,N6 | LAW_003_DOCTRINE_FAIL | LawValidationRejected, DeploymentRejected |
| LAW-04 | Registry conformance (modules must exist + ACTIVE + compatible) | MUST verify module records + compat | MUST deny deploy if any module not ACTIVE/compatible | N/A | Registry | N3,N4,N6 | LAW_004_REGISTRY_FAIL | ModuleStatusChanged, DeploymentRejected |
| LAW-05 | Enforcement matrix compile must succeed | MUST compile invariant coverage map | MUST deny deploy if compile failed | SHOULD store compiled hash | Compiler | N5,N6 | LAW_005_MATRIX_COMPILE_FAIL | LawValidationPassed OR LawValidationRejected |
| LAW-06 | No bypass paths for deployment | N/A | MUST ensure only factory path can create assets | SHOULD assert factory-only init | Factory | N6,N8,N9 | RCD_002_NO_BYPASS (if used) | AssetDeployed, DeploymentRejected |
| LAW-07 | Immutable audit trail (reconstructible decisions) | SHOULD emit start/end validation events | SHOULD emit allow/reject events | SHOULD emit runtime critical events | All | N11 | N/A | All listed events throughout matrix |

Notes:
- LAW-06 reject code can reuse existing RCD scheme (example: `RCD_002_NO_BYPASS`) or you can keep it informational in v0.
- LAW-07 is auditability: not always a revert condition, but REQUIRED for credibility.

---

# 3) Module Enforcement Matrix (v0)
## Module 1–7 + Module 8 OIM

---

## 3.1 Module 1: FixedSupply

| Invariant ID | Invariant (summary) | EP-1 Compile | EP-2 Deploy | EP-3 Runtime Assert | Node Owner(s) | Reject Code | Required Events |
|---|---|---|---|---|---|---|---|
| M1-01 | Total supply fixed at genesis | MUST confirm supply specified + immutable flags | MUST deny deploy if missing/invalid | MUST forbid mint | N4,N6,N9 | MOD1_001_SUPPLY_INVALID | AssetDeployed, RuntimeInvariantViolated(if any) |

Runtime assertions:
- No mint entrypoints OR mint permanently disabled
- total_supply stored once at init; no setter

---

## 3.2 Module 2: BurnCap

| Invariant ID | Invariant (summary) | EP-1 Compile | EP-2 Deploy | EP-3 Runtime Assert | Node Owner(s) | Reject Code | Required Events |
|---|---|---|---|---|---|---|---|
| M2-01 | Burn cannot exceed cap | MUST validate cap present | MUST deny deploy if cap missing/invalid | MUST revert burns beyond cap | N4,N6,N9 | MOD2_001_BURNCAP_INVALID | BurnExecuted, RuntimeInvariantViolated |

---

## 3.3 Module 3: TimeLock

| Invariant ID | Invariant (summary) | EP-1 Compile | EP-2 Deploy | EP-3 Runtime Assert | Node Owner(s) | Reject Code | Required Events |
|---|---|---|---|---|---|---|---|
| M3-01 | Deterministic unlock windows | MUST validate schedule structure | MUST deny deploy if schedule invalid | MUST enforce unlock windows | N4,N6,N9 | MOD3_001_TIMELOCK_INVALID | TimelockConfigured, UnlockWindowOpened |

---

## 3.4 Module 4: Airdrop

| Invariant ID | Invariant (summary) | EP-1 Compile | EP-2 Deploy | EP-3 Runtime Assert | Node Owner(s) | Reject Code | Required Events |
|---|---|---|---|---|---|---|---|
| M4-01 | Airdrop rules deterministic | MUST validate recipient selection constraints | MUST deny deploy if missing | MUST enforce eligibility checks | N4,N6,N9 | MOD4_001_AIRDROP_INVALID | AirdropScheduled, AirdropExecuted |

---

## 3.5 Module 5: VestingSchedule

| Invariant ID | Invariant (summary) | EP-1 Compile | EP-2 Deploy | EP-3 Runtime Assert | Node Owner(s) | Reject Code | Required Events |
|---|---|---|---|---|---|---|---|
| M5-01 | Vesting deterministic | MUST validate vesting plan | MUST deny deploy if invalid | MUST enforce vesting claims | N4,N6,N9 | MOD5_001_VESTING_INVALID | VestingConfigured, VestingClaimed |

---

## 3.6 Module 6: LiquidityBoost

| Invariant ID | Invariant (summary) | EP-1 Compile | EP-2 Deploy | EP-3 Runtime Assert | Node Owner(s) | Reject Code | Required Events |
|---|---|---|---|---|---|---|---|
| M6-01 | Liquidity actions bounded by caps | MUST validate LP cap params | MUST deny deploy if invalid | MUST enforce caps on LP actions | N4,N6,N9 | MOD6_001_LP_INVALID | LiquidityConfigured, LiquidityAction |

---

## 3.7 Module 7: PercentageDistribution

| Invariant ID | Invariant (summary) | EP-1 Compile | EP-2 Deploy | EP-3 Runtime Assert | Node Owner(s) | Reject Code | Required Events |
|---|---|---|---|---|---|---|---|
| M7-01 | Distribution sums to 100% (or declared total) | MUST validate sum + bucket caps | MUST deny deploy if invalid | MUST enforce allocations | N4,N6,N9 | MOD7_001_DIST_INVALID | DistributionConfigured, DistributionExecuted |

---

# 4) Module 8: OutpaceInflation (OIM) Enforcement Matrix (v0)

OIM invariants are subordinate to LAW invariants and MUST NOT modify supply or timelocks.

Legend:
- Oracle role is data-only (posts inflation index), never funds.
- OIM rebalance is governance-controlled, bounded, and deterministic.

| Invariant ID | Invariant (summary) | EP-1 Compile Gate | EP-2 Deploy Gate | EP-3 Runtime Assert | Primary Owner | Node Owner(s) | Reject/Fail Code | Required Events |
|---|---|---|---|---|---|---|---|---|
| INV-OIM-01 | Supply immutability | MUST confirm OIM has no mint/burn permissions | MUST deny deploy if OIM implies supply mutation | MUST assert no supply change paths exist | OIM + FixedSupply | N4,N6,N9,N12 | OIM_001_SUPPLY_TOUCH | RuntimeInvariantViolated |
| INV-OIM-02 | Timelock immutability | MUST confirm OIM cannot call/modify timelock state | MUST deny deploy if binding violates | MUST assert no timelock mutation | OIM + TimeLock/Vesting | N4,N6,N9,N12 | OIM_002_TIMELOCK_TOUCH | RuntimeInvariantViolated |
| INV-OIM-03 | Bounded rebalance | MUST validate bucket caps + step caps present | MUST deny deploy if caps invalid | MUST enforce caps and max step | OIM | N4,N6,N9,N12 | OIM_003_BOUNDS_FAIL | OIM_REBALANCED, RuntimeInvariantViolated |
| INV-OIM-04 | Oracle cannot move funds | MUST validate oracle method is data-only | MUST deny deploy if oracle has exec hooks | MUST assert oracle cannot trigger rebalance | OIM | N4,N6,N9,N12 | OIM_004_ORACLE_PRIV | OIM_INDEX_POSTED |
| INV-OIM-05 | Oracle staleness fail-safe | MUST validate max_oracle_age_sec present (oracle mode) | MUST deny deploy if missing | MUST freeze rebalance when stale | OIM | N4,N6,N12 | OIM_005_ORACLE_STALE | OIM_STATUS_UPDATED |
| INV-OIM-06 | Cooldown enforced | MUST validate cooldown param | MUST deny deploy if missing/invalid | MUST enforce cooldown between rebalances | OIM | N4,N6,N12 | OIM_006_COOLDOWN | OIM_REBALANCED |
| INV-OIM-07 | Deterministic accounting only | MUST ensure scoring uses on-chain accounting only | MUST deny deploy if price-feed scoring referenced | MUST assert scoring does not read market prices | OIM | N4,N6,N12 | OIM_007_NONDET_SCORE | OIM_STATUS_UPDATED |
| INV-OIM-08 | Governance boundary | MUST bind rebalance to governance/multisig only | MUST deny deploy if callable by arbitrary user | MUST assert predecessor auth on rebalance | OIM + Governance | N4,N6,N9,N10,N12 | OIM_008_GOV_BOUNDARY | OIM_REBALANCED, RuntimeInvariantViolated |

---

# 5) Canonical Rejection / Reason Code Namespace (v0)

LAW (from LAW_VALIDATION.md):
- LAW_001_SCHEMA_INVALID
- LAW_002_INTEGRITY_FAIL
- LAW_003_DOCTRINE_FAIL
- LAW_004_REGISTRY_FAIL
- LAW_005_MATRIX_COMPILE_FAIL

RCD (from REGISTRY_CERTIFICATION_DEPLOYMENT.md):
- RCD_001_LAW_INVALID
- RCD_002_OIM_MISSING
- RCD_003_STACK_RULE_VIOLATION
- RCD_004_MODULE_NOT_ACTIVE
- RCD_005_LAW_INCOMPATIBLE
- RCD_006_INVARIANT_COVERAGE_FAIL

Module codes (recommended):
- MOD1_001_SUPPLY_INVALID
- MOD2_001_BURNCAP_INVALID
- MOD3_001_TIMELOCK_INVALID
- MOD4_001_AIRDROP_INVALID
- MOD5_001_VESTING_INVALID
- MOD6_001_LP_INVALID
- MOD7_001_DIST_INVALID

OIM codes (recommended):
- OIM_001_SUPPLY_TOUCH
- OIM_002_TIMELOCK_TOUCH
- OIM_003_BOUNDS_FAIL
- OIM_004_ORACLE_PRIV
- OIM_005_ORACLE_STALE
- OIM_006_COOLDOWN
- OIM_007_NONDET_SCORE
- OIM_008_GOV_BOUNDARY

---

# 6) Required Event Set (v0)

Control Plane:
- ModuleSubmitted
- ModuleCertified
- ModuleStatusChanged
- LawValidationStarted
- LawValidationPassed
- LawValidationRejected
- AssetDeployed
- DeploymentRejected

Runtime (minimum):
- RuntimeInvariantViolated (generic; include invariant_id + at)
- OIM_INDEX_POSTED
- OIM_STATUS_UPDATED
- OIM_REBALANCED

---

# 7) Auditor Checklist Mapping (v0)

An auditor should be able to verify:

A) No deployment bypass exists:
- Only deployments passing N4/N6 can create AssetRecord + instantiate contract.

B) LAW validation is deterministic:
- Given same LAW profile + registry state → same allow/deny result.

C) OIM cannot touch supply or timelocks:
- No cross-calls or mutable interfaces exist.

D) Oracle is data-only:
- Oracle can post inflation, cannot rebalance or transfer funds.

E) Every rejection is explainable:
- reason_code + details_hash always emitted on rejects.

---

END ENFORCEMENT_MATRIX_v0

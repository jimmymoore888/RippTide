use near_sdk::{
    env, near_bindgen, AccountId, PanicOnDefault, Balance,
    BorshDeserialize, BorshSerialize, require,
};
use near_sdk::collections::LookupMap;

// ===== IMPORT YOUR LAW LAYER =====
mod law;
use law::{
    LawV1Schema, FixedSupply, BurnCap, Airdrop,
    TimeLock, VestingSchedule
};

// ===== CONSTANTS =====
const MAX_SUPPLY: Balance = 10_000_000_000;
const REQUIRED_MODULES: u8 = 4;

// ===== TOKEN ID =====
pub type TokenId = String;

// ===== TOKEN STRUCT =====
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    pub owner: AccountId,
    pub law: LawV1Schema,
}

// ===== CONTRACT =====
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NearIntersect {
    pub owner_id: AccountId,
    pub tokens: LookupMap<TokenId, Token>,
}

#[near_bindgen]
impl NearIntersect {

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        require!(!env::state_exists(), "Already initialized");

        Self {
            owner_id,
            tokens: LookupMap::new(b"t"),
        }
    }

    // ===== FACTORY ENTRY (ENFORCED) =====
    #[payable]
    pub fn create_token(
        &mut self,
        token_id: TokenId,
        law: LawV1Schema,
    ) {
        let caller = env::predecessor_account_id();
        let initial_storage = env::storage_usage();

        require!(
            !self.tokens.contains_key(&token_id),
            "Token already exists"
        );

        // ===== ENFORCE LAW =====
        self.enforce_law(&law);

        // ===== CREATE TOKEN =====
        let token = Token {
            owner: caller.clone(),
            law: law.clone(),
        };

        self.tokens.insert(&token_id, &token);

        // ===== STORAGE CHECK =====
        let used = env::storage_usage() - initial_storage;
        let required_cost = Balance::from(used) * env::storage_byte_cost();

        require!(
            env::attached_deposit() >= required_cost,
            "Insufficient deposit"
        );

        // ===== EVENT LOG =====
        env::log_str(
            &format!(
                "EVENT_JSON:{{\"event\":\"create_token\",\"token_id\":\"{}\",\"owner\":\"{}\"}}",
                token_id, caller
            )
        );
    }

    // ===== LAW ENFORCEMENT CORE =====
    fn enforce_law(&self, law: &LawV1Schema) {

        // --- FIXED SUPPLY MUST MATCH GLOBAL LAW ---
        require!(
            law.fixed_supply.total == MAX_SUPPLY,
            "Fixed supply violation"
        );

        // --- BURN CAP VALIDATION ---
        require!(
            law.burn_cap.cap <= MAX_SUPPLY,
            "Burn cap exceeds supply"
        );

        // --- OIM / TIMELOCK ENFORCEMENT ---
        require!(
            law.time_lock.cycles > 0,
            "TimeLock must exist"
        );

        // --- VESTING SANITY ---
        require!(
            law.vesting.total_duration > 0,
            "Invalid vesting schedule"
        );

        // --- AIRDROP VALIDATION ---
        require!(
            law.airdrop.total_percentage <= 100,
            "Invalid airdrop percentage"
        );

        // --- STRUCTURAL COMPLETENESS ---
        require!(
            self.validate_schema_completeness(law),
            "Incomplete law schema"
        );
    }

    // ===== SCHEMA COMPLETENESS CHECK =====
    fn validate_schema_completeness(&self, law: &LawV1Schema) -> bool {
        // ensures no “empty shell” configs
        law.fixed_supply.total > 0 &&
        law.time_lock.cycles > 0 &&
        law.vesting.total_duration > 0
    }

    // ===== VIEW =====
    pub fn get_token(&self, token_id: TokenId) -> Option<LawV1Schema> {
        self.tokens.get(&token_id).map(|t| t.law)
    }
}

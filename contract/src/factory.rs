use crate::law::LawV1Schema;
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, AccountId};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Factory {
    registry: LookupMap<String, TokenRecord>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenRecord {
    pub account: AccountId,
    pub law: LawV1Schema,
    pub created_by: AccountId,
    pub created_at_ns: u64,
    /// sha256(borsh(law)) — binds schema to a deterministic fingerprint
    pub schema_hash: Vec<u8>,
}

impl Factory {
    pub fn new() -> Self {
        Self {
            registry: LookupMap::new(b"r"),
        }
    }

    pub fn create_token(&mut self, symbol: String, account: AccountId, law: LawV1Schema) {
        let s = normalize_symbol(&symbol);

        assert!(self.registry.get(&s).is_none(), "SYMBOL_USED");

        // LAW enforcement (hard fail)
        law.validate();

        // schema_hash = sha256(borsh(law))
        let schema_bytes = law.try_to_vec().expect("LAW: borsh serialize failed");
        let schema_hash = env::sha256(&schema_bytes);

        let rec = TokenRecord {
            account,
            law,
            created_by: env::predecessor_account_id(),
            created_at_ns: env::block_timestamp(),
            schema_hash,
        };

        self.registry.insert(&s, &rec);
    }

    pub fn get_token(&self, symbol: String) -> Option<AccountId> {
        let s = normalize_symbol(&symbol);
        self.registry.get(&s).map(|r| r.account)
    }

    pub fn get_record(&self, symbol: String) -> Option<TokenRecord> {
        let s = normalize_symbol(&symbol);
        self.registry.get(&s)
    }
}

fn normalize_symbol(symbol: &str) -> String {
    symbol.trim().to_uppercase()
}

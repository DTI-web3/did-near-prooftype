use near_sdk::{env, log, near, serde, store::LookupMap, AccountId};

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct VerifiableCredential {
    subject_did: String,
    issuer: AccountId,
    cid: String,
    issued_at: u64,
    expires_at: Option<u64>,
    revoked: bool,
}

#[derive(serde::Serialize, schemars::JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct VerifiableCredentialOutput {
    pub subject_did: String,
    pub issuer: String,
    pub cid: String,
    pub issued_at: u64,
    pub expires_at: Option<u64>,
    pub revoked: bool,
}

#[near(contract_state)]
pub struct NearProofType2025 {
    credentials: LookupMap<String, VerifiableCredential>,
}

impl Default for NearProofType2025 {
    fn default() -> Self {
        Self { 
            credentials: LookupMap::new(b"c")
        }
    }
}

#[near]
impl NearProofType2025 {
    pub fn issue_credential(&mut self, subject_did: String, cid: String, expires_at: Option<u64>) {
        assert!(!subject_did.is_empty(), "Empty subject DID not allowed");
        assert!(cid.len() > 10, "CID too short");
        let issuer = env::predecessor_account_id();
        let id = format!("{}:{}", subject_did, cid);
        assert!(
            !self.credentials.contains_key(&id),
            "Credential already issued"
        );

        let credential = VerifiableCredential {
            subject_did,
            issuer,
            cid,
            issued_at: env::block_timestamp_ms(),
            expires_at,
            revoked: false,
        };

        self.credentials.insert(id, credential);
    }

    pub fn revoke_credential(&mut self, subject_did: String, cid: String) {
        assert!(!subject_did.is_empty(), "Empty subject DID not allowed");
        assert!(cid.len() > 10, "CID too short");
        let id = format!("{}:{}", subject_did, cid);
        let mut credential = self
            .credentials
            .get(&id).cloned()
            .expect("Credential not found");

        assert_eq!(
            env::predecessor_account_id(),
            credential.issuer,
            "Only issuer can revoke"
        );

        credential.revoked = true;
        self.credentials.insert(id, credential);
    }

    pub fn is_valid(&self, subject_did: String, cid: String) -> bool {
        assert!(!subject_did.is_empty(), "Empty subject DID not allowed");
        assert!(cid.len() > 10, "CID too short");
        let id = format!("{}:{}", subject_did, cid);
        if let Some(cred) = self.credentials.get(&id) {
            if cred.revoked {
                return false;
            }
            if let Some(exp) = cred.expires_at {
                return env::block_timestamp_ms() < exp;
            }
            return true;
        }
        false
    }

    pub fn get_credential(&self, subject_did: String, cid: String) -> Option<VerifiableCredentialOutput> {
        assert!(!subject_did.is_empty(), "Empty subject DID not allowed");
        assert!(cid.len() > 10, "CID too short");
        let id = format!("{}:{}", subject_did, cid);
        self.credentials.get(&id).cloned().map(|cred| {
            cred.into()
        })
    }
}

impl From<VerifiableCredential> for VerifiableCredentialOutput {
    fn from(cred: VerifiableCredential) -> Self {
        Self {
            subject_did: cred.subject_did,
            issuer: cred.issuer.to_string(),
            cid: cred.cid,
            issued_at: cred.issued_at,
            expires_at: cred.expires_at,
            revoked: cred.revoked,
        }
    }
}
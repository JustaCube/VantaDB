use chrono::{Duration, Utc};
use dashmap::DashSet;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::io;
use std::sync::Arc;

use crate::auth::Role;
use crate::storage::StorageEngine;

const SESSION_TABLE: &str = "_vanta_sessions";
const JWT_SECRET_KEY: &str = "_jwt_secret";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,       // username
    pub role: String,      // role string
    pub exp: usize,        // expiry (unix timestamp)
    pub iat: usize,        // issued at
    pub jti: String,       // unique token ID for revocation
}

pub struct JwtSessionManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    ttl_hours: i64,
    revoked: DashSet<String>,  // revoked jti values
}

impl JwtSessionManager {
    pub fn new(ttl_hours: i64) -> Self {
        let secret = generate_secret();
        Self::from_secret(secret, ttl_hours)
    }

    pub fn open(engine: Arc<StorageEngine>, ttl_hours: i64) -> io::Result<Self> {
        if !engine.table_exists(SESSION_TABLE) {
            engine.create_table(SESSION_TABLE)?;
        }

        let secret = match engine.get(SESSION_TABLE, JWT_SECRET_KEY) {
            Some(existing) => existing.to_vec(),
            None => {
                let generated = generate_secret();
                engine.put(SESSION_TABLE, JWT_SECRET_KEY, &generated)?;
                generated
            }
        };

        Ok(Self::from_secret(secret, ttl_hours))
    }

    fn from_secret(secret: Vec<u8>, ttl_hours: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(&secret),
            decoding_key: DecodingKey::from_secret(&secret),
            ttl_hours,
            revoked: DashSet::new(),
        }
    }

    pub fn create_token(&self, username: &str, role: &Role) -> String {
        let now = Utc::now();
        let exp = now + Duration::hours(self.ttl_hours);
        let jti = uuid::Uuid::new_v4().to_string();

        let claims = Claims {
            sub: username.to_string(),
            role: role.to_string(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
            jti,
        };

        let header = Header::new(Algorithm::HS512);
        encode(&header, &claims, &self.encoding_key)
            .expect("JWT encoding should not fail")
    }

    pub fn validate(&self, token: &str) -> Option<(String, Role)> {
        let mut validation = Validation::new(Algorithm::HS512);
        validation.validate_exp = true;
        let token_data: TokenData<Claims> = decode(
            token,
            &self.decoding_key,
            &validation,
        ).ok()?;

        let claims = token_data.claims;

        // Check revocation
        if self.revoked.contains(&claims.jti) {
            return None;
        }

        let role = Role::from_str(&claims.role)?;
        Some((claims.sub, role))
    }

    pub fn revoke(&self, token: &str) {
        let validation = Validation::new(Algorithm::HS512);
        if let Ok(token_data) = decode::<Claims>(
            token,
            &self.decoding_key,
            &validation,
        ) {
            self.revoked.insert(token_data.claims.jti);
        }
    }
}

fn generate_secret() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..64).map(|_| rng.gen::<u8>()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_and_validate_token() {
        let mgr = JwtSessionManager::new(24);
        let token = mgr.create_token("alice", &Role::Admin);
        let (username, role) = mgr.validate(&token).unwrap();
        assert_eq!(username, "alice");
        assert_eq!(role, Role::Admin);
    }

    #[test]
    fn test_invalid_token_rejected() {
        let mgr = JwtSessionManager::new(24);
        assert!(mgr.validate("garbage.token.here").is_none());
    }

    #[test]
    fn test_revoke_token() {
        let mgr = JwtSessionManager::new(24);
        let token = mgr.create_token("bob", &Role::ReadWrite);
        assert!(mgr.validate(&token).is_some());

        mgr.revoke(&token);
        assert!(mgr.validate(&token).is_none());
    }

    #[test]
    fn test_different_tokens_per_call() {
        let mgr = JwtSessionManager::new(24);
        let t1 = mgr.create_token("alice", &Role::Root);
        let t2 = mgr.create_token("alice", &Role::Root);
        assert_ne!(t1, t2);
    }

    #[test]
    fn test_persistent_secret_survives_restart() {
        let dir = TempDir::new().unwrap();
        let engine = Arc::new(StorageEngine::open(dir.path()).unwrap());

        let mgr = JwtSessionManager::open(Arc::clone(&engine), 24).unwrap();
        let token = mgr.create_token("alice", &Role::Admin);
        drop(mgr);
        drop(engine);

        let reopened_engine = Arc::new(StorageEngine::open(dir.path()).unwrap());
        let reopened_mgr = JwtSessionManager::open(reopened_engine, 24).unwrap();

        let (username, role) = reopened_mgr.validate(&token).unwrap();
        assert_eq!(username, "alice");
        assert_eq!(role, Role::Admin);
    }
}

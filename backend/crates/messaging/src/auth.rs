use std::{collections::HashMap, fmt};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use payment_core::{AgentId, AgentMessageEnvelope, MessagePrincipal};
use sha2::Sha256;
use thiserror::Error;

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct MessageKey {
    pub key_id: String,
    pub principal: MessagePrincipal,
    secret: Vec<u8>,
    pub revoked: bool,
}

impl MessageKey {
    pub fn new(
        key_id: impl Into<String>,
        principal: MessagePrincipal,
        secret: Vec<u8>,
    ) -> Result<Self, MessageAuthError> {
        let key_id = key_id.into();
        if key_id.is_empty()
            || key_id.len() > 64
            || !key_id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        {
            return Err(MessageAuthError::InvalidKey);
        }
        if secret.len() < 32 {
            return Err(MessageAuthError::WeakKey);
        }
        Ok(Self {
            key_id,
            principal,
            secret,
            revoked: false,
        })
    }

    pub fn revoke(mut self) -> Self {
        self.revoked = true;
        self
    }
}

impl fmt::Debug for MessageKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MessageKey")
            .field("key_id", &self.key_id)
            .field("principal", &self.principal)
            .field("secret", &"[REDACTED]")
            .field("revoked", &self.revoked)
            .finish()
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MessageAuthError {
    #[error("Unknown or revoked signing key")]
    UnknownKey,
    #[error("Signing key does not identify the declared sender")]
    SenderMismatch,
    #[error("Message signature is invalid")]
    InvalidSignature,
    #[error("Signing key must contain at least 32 bytes")]
    WeakKey,
    #[error("Signing key identifier is invalid")]
    InvalidKey,
    #[error("Message encoding failed")]
    Encoding,
}

#[derive(Clone, Default)]
pub struct MessageKeyring {
    keys: HashMap<String, MessageKey>,
}

impl fmt::Debug for MessageKeyring {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MessageKeyring")
            .field("key_ids", &self.keys.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl MessageKeyring {
    pub fn from_json(encoded_keys: &str) -> Result<Self, MessageAuthError> {
        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct KeyConfig {
            key_id: String,
            principal: String,
            secret_base64: String,
            #[serde(default)]
            revoked: bool,
        }

        let configs: Vec<KeyConfig> =
            serde_json::from_str(encoded_keys).map_err(|_| MessageAuthError::Encoding)?;
        if configs.is_empty() || configs.iter().all(|config| config.revoked) {
            return Err(MessageAuthError::InvalidKey);
        }
        let keys = configs
            .into_iter()
            .map(|config| {
                let principal = parse_principal(&config.principal)?;
                let secret = URL_SAFE_NO_PAD
                    .decode(config.secret_base64)
                    .map_err(|_| MessageAuthError::Encoding)?;
                let mut key = MessageKey::new(config.key_id, principal, secret)?;
                key.revoked = config.revoked;
                Ok(key)
            })
            .collect::<Result<Vec<_>, MessageAuthError>>()?;
        Self::new(keys)
    }

    pub fn new(keys: impl IntoIterator<Item = MessageKey>) -> Result<Self, MessageAuthError> {
        let mut keyring = Self::default();
        for key in keys {
            if keyring.keys.contains_key(&key.key_id) {
                return Err(MessageAuthError::InvalidKey);
            }
            keyring.keys.insert(key.key_id.clone(), key);
        }
        Ok(keyring)
    }

    pub fn sign(
        &self,
        envelope: &mut AgentMessageEnvelope,
        key_id: &str,
    ) -> Result<(), MessageAuthError> {
        let key = self.keys.get(key_id).ok_or(MessageAuthError::UnknownKey)?;
        if key.revoked {
            return Err(MessageAuthError::UnknownKey);
        }
        if key.principal != envelope.sender_id {
            return Err(MessageAuthError::SenderMismatch);
        }
        envelope.key_id = key_id.to_owned();
        envelope.signature.clear();
        let bytes = signing_bytes(envelope)?;
        let mut mac =
            HmacSha256::new_from_slice(&key.secret).map_err(|_| MessageAuthError::InvalidKey)?;
        mac.update(&bytes);
        envelope.signature = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());
        Ok(())
    }

    pub fn verify(
        &self,
        envelope: &AgentMessageEnvelope,
    ) -> Result<MessagePrincipal, MessageAuthError> {
        let key = self
            .keys
            .get(&envelope.key_id)
            .filter(|key| !key.revoked)
            .ok_or(MessageAuthError::UnknownKey)?;
        if key.principal != envelope.sender_id {
            return Err(MessageAuthError::SenderMismatch);
        }
        let signature = URL_SAFE_NO_PAD
            .decode(&envelope.signature)
            .map_err(|_| MessageAuthError::InvalidSignature)?;
        let bytes = signing_bytes(envelope)?;
        let mut mac =
            HmacSha256::new_from_slice(&key.secret).map_err(|_| MessageAuthError::InvalidKey)?;
        mac.update(&bytes);
        mac.verify_slice(&signature)
            .map_err(|_| MessageAuthError::InvalidSignature)?;
        Ok(key.principal)
    }
}

fn parse_principal(value: &str) -> Result<MessagePrincipal, MessageAuthError> {
    match value {
        "API" => Ok(MessagePrincipal::Api),
        "ORCHESTRATOR" => Ok(MessagePrincipal::Orchestrator),
        _ => {
            let agent_id = match value.strip_prefix("AGENT:") {
                Some("master_orchestrator") => AgentId::MasterOrchestrator,
                Some("transaction_analysis") => AgentId::TransactionAnalysis,
                Some("identity_verification") => AgentId::IdentityVerification,
                Some("merchant_risk") => AgentId::MerchantRisk,
                Some("device_session") => AgentId::DeviceSession,
                Some("behavioral_analysis") => AgentId::BehavioralAnalysis,
                Some("payment_analysis") => AgentId::PaymentAnalysis,
                Some("checkout_optimization") => AgentId::CheckoutOptimization,
                Some("customer_intelligence") => AgentId::CustomerIntelligence,
                Some("recommendations") => AgentId::Recommendations,
                Some("refund_analysis") => AgentId::RefundAnalysis,
                Some("dispute_investigation") => AgentId::DisputeInvestigation,
                Some("business_ops") => AgentId::BusinessOps,
                _ => return Err(MessageAuthError::InvalidKey),
            };
            Ok(MessagePrincipal::Agent(agent_id))
        }
    }
}

fn signing_bytes(envelope: &AgentMessageEnvelope) -> Result<Vec<u8>, MessageAuthError> {
    let mut unsigned = envelope.clone();
    unsigned.signature.clear();
    serde_json::to_vec(&unsigned).map_err(|_| MessageAuthError::Encoding)
}

#[cfg(test)]
mod tests {
    use super::*;
    use payment_core::{
        AgentId, AgentMessagePayload, AgentMessageType, AgentPayloadType, TenantId, Timestamp,
    };
    use uuid::Uuid;

    fn key(principal: MessagePrincipal) -> MessageKey {
        MessageKey::new("agent-key-1", principal, vec![7; 32]).expect("valid key")
    }

    fn envelope(principal: MessagePrincipal) -> AgentMessageEnvelope {
        let tenant_id = TenantId(Uuid::new_v4());
        let task_id = Uuid::new_v4();
        AgentMessageEnvelope {
            message_id: Uuid::new_v4(),
            task_id,
            tenant_id,
            sender_id: principal,
            recipient_id: MessagePrincipal::Orchestrator,
            message_type: AgentMessageType::AgentTaskFailed,
            payload_type: AgentPayloadType::AgentFailure,
            schema_version: "1.0".into(),
            issued_at: Timestamp::now(),
            expires_at: Timestamp::now(),
            correlation_id: Uuid::new_v4(),
            trace_id: Uuid::new_v4(),
            payload: AgentMessagePayload::AgentFailure {
                task_id,
                agent_id: AgentId::TransactionAnalysis,
                failure_category: "test".into(),
                retryable: false,
                safe_diagnostic: "test failure".into(),
            },
            key_id: String::new(),
            signature: String::new(),
        }
    }

    #[test]
    fn signs_and_verifies_message_with_key_identity() {
        let principal = MessagePrincipal::Agent(AgentId::TransactionAnalysis);
        let keyring = MessageKeyring::new([key(principal)]).expect("keyring");
        let mut message = envelope(principal);
        keyring.sign(&mut message, "agent-key-1").expect("sign");

        assert_eq!(keyring.verify(&message), Ok(principal));
    }

    #[test]
    fn rejects_tampered_message() {
        let principal = MessagePrincipal::Agent(AgentId::TransactionAnalysis);
        let keyring = MessageKeyring::new([key(principal)]).expect("keyring");
        let mut message = envelope(principal);
        keyring.sign(&mut message, "agent-key-1").expect("sign");
        message.task_id = Uuid::new_v4();

        assert_eq!(
            keyring.verify(&message),
            Err(MessageAuthError::InvalidSignature)
        );
    }

    #[test]
    fn rejects_sender_spoofing_and_revoked_keys() {
        let principal = MessagePrincipal::Agent(AgentId::TransactionAnalysis);
        let keyring = MessageKeyring::new([key(principal)]).expect("keyring");
        let mut message = envelope(MessagePrincipal::Orchestrator);
        assert_eq!(
            keyring.sign(&mut message, "agent-key-1"),
            Err(MessageAuthError::SenderMismatch)
        );

        let revoked = MessageKeyring::new([key(principal).revoke()]).expect("keyring");
        let mut message = envelope(principal);
        assert_eq!(
            revoked.sign(&mut message, "agent-key-1"),
            Err(MessageAuthError::UnknownKey)
        );
    }

    #[test]
    fn rejects_short_signing_keys() {
        assert_eq!(
            MessageKey::new("weak", MessagePrincipal::Orchestrator, vec![0; 31])
                .expect_err("short key must fail"),
            MessageAuthError::WeakKey
        );
    }

    #[test]
    fn loads_only_valid_base64_key_configuration() {
        let secret = URL_SAFE_NO_PAD.encode([7; 32]);
        let keys =
            format!(r#"[{{"key_id":"api-1","principal":"API","secret_base64":"{secret}"}}]"#);
        assert!(MessageKeyring::from_json(&keys).is_ok());
        assert!(
            MessageKeyring::from_json(
                r#"[{"key_id":"api-1","principal":"API","secret_base64":"not-base64"}]"#
            )
            .is_err()
        );
    }
}

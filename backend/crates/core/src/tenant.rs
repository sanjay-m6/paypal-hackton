use serde::{Deserialize, Serialize};

use crate::types::{TenantId, Timestamp, UserId};

/// A registered tenant on the platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: TenantId,
    pub name: String,
    pub active: bool,
    pub created_at: Timestamp,
}

/// Per-request context identifying the tenant, user, and their permissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantContext {
    pub tenant_id: TenantId,
    pub user_id: UserId,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

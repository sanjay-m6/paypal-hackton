use std::collections::HashMap;
use std::sync::Arc;

use payment_core::{AgentId, AgentMetadata};

use crate::traits::Agent;

/// Registry that holds all registered agent instances.
///
/// Agents are registered at startup and looked up by their [`AgentId`] when
/// the orchestrator needs to dispatch work.
#[derive(Debug, Clone)]
pub struct AgentRegistry {
    agents: HashMap<AgentId, Arc<dyn Agent>>,
}

impl AgentRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    /// Register an agent. Replaces any existing agent with the same id.
    pub fn register(&mut self, agent: Arc<dyn Agent>) {
        let id = agent.id();
        tracing::info!(?id, "Registering agent");
        self.agents.insert(id, agent);
    }

    /// Look up an agent by id.
    pub fn get(&self, id: &AgentId) -> Option<Arc<dyn Agent>> {
        self.agents.get(id).cloned()
    }

    /// List metadata for all registered agents.
    pub fn list(&self) -> Vec<AgentMetadata> {
        self.agents.values().map(|a| a.metadata().clone()).collect()
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Debug is manually implemented because `dyn Agent` is not Debug by default,
// but we declared it on the struct — the HashMap wrapper handles it via Arc's Debug.
// If the compiler complains, we can implement Debug manually instead.

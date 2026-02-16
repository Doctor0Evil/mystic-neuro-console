use serde::{Deserialize, Serialize};
use crate::command::{Command, CommandKind};
use crate::role::Role;
use crate::scope::Scope;
use thiserror::Error;
use std::collections::HashMap;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("permission denied for role {0:?}")]
    PermissionDenied(Role),
    #[error("invalid scope for command")]
    InvalidScope,
    #[error("general backend error: {0}")]
    General(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub command_id: uuid::Uuid,
    pub ok: bool,
    pub message: String,
}

#[derive(Debug, Default)]
pub struct NeuromorphicBackend {
    cache_cleared: bool,
    context_size: usize,
    sessions: HashMap<String, String>,
    codex_valid: bool,
}

impl NeuromorphicBackend {
    pub fn new() -> Self {
        Self {
            cache_cleared: false,
            context_size: 1024,
            sessions: HashMap::new(),
            codex_valid: true,
        }
    }

    pub fn handle(&mut self, cmd: Command) -> Result<CommandResult, BackendError> {
        // very simple role check: admin-only for destructive ops
        match cmd.kind {
            CommandKind::NeuralCachePurge
            | CommandKind::SessionArchive
            | CommandKind::SessionSnapshot
            | CommandKind::SessionRecover => {
                if !cmd.role.can_execute_admin_ops() {
                    return Err(BackendError::PermissionDenied(cmd.role));
                }
            }
            _ => {}
        }

        let message = match cmd.kind {
            CommandKind::NeuralCachePurge => {
                self.cache_cleared = true;
                "neural cache purged".to_string()
            }
            CommandKind::ContextExpand { amount } => {
                self.context_size += amount;
                format!("context expanded by {}, new size {}", amount, self.context_size)
            }
            CommandKind::SessionArchive => match cmd.scope {
                Scope::Session { ref id } => {
                    self.sessions.insert(id.clone(), "archived".into());
                    format!("session {} archived", id)
                }
                _ => return Err(BackendError::InvalidScope),
            },
            CommandKind::SessionSnapshot => match cmd.scope {
                Scope::Session { ref id } => {
                    self.sessions.insert(id.clone(), "snapshot".into());
                    format!("session {} snapshot taken", id)
                }
                _ => return Err(BackendError::InvalidScope),
            },
            CommandKind::SessionRecover => match cmd.scope {
                Scope::Session { ref id } => {
                    if self.sessions.contains_key(id) {
                        format!("session {} recovered", id)
                    } else {
                        format!("session {} not found; nothing to recover", id)
                    }
                }
                _ => return Err(BackendError::InvalidScope),
            },
            CommandKind::CodexValidate => {
                self.codex_valid = true;
                "codex validated".to_string()
            }
            CommandKind::CodexCompress => "codex compressed (logical operation)".to_string(),
            CommandKind::NodeRebalance => "node workloads rebalanced (simulated)".to_string(),
            CommandKind::QueryTrace => "query trace recorded (simulated)".to_string(),
            CommandKind::QueryOptimize => "query execution paths optimized (simulated)".to_string(),
        };

        Ok(CommandResult {
            command_id: cmd.id,
            ok: true,
            message,
        })
    }
}

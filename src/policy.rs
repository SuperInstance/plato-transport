use std::collections::HashSet;
use std::sync::Mutex;
use std::time::Instant;

/// Rate limiting configuration.
#[derive(Debug, Clone)]
pub struct RateLimit {
    /// Maximum number of commands allowed within the window.
    pub max_commands: u32,
    /// Window duration in milliseconds.
    pub window_ms: u64,
}

/// Policy controlling what commands are allowed and at what rate.
#[derive(Debug)]
pub struct TransportPolicy {
    /// If Some, only these command prefixes are allowed. If None, all are allowed.
    allowed_prefixes: Option<HashSet<String>>,
    /// If Some, rate limiting is enforced.
    rate_limit: Option<RateLimit>,
    /// Timestamps of recent commands for rate limiting.
    command_times: Mutex<Vec<Instant>>,
}

impl TransportPolicy {
    /// Create a permissive policy that allows everything.
    pub fn allow_all() -> Self {
        Self {
            allowed_prefixes: None,
            rate_limit: None,
            command_times: Mutex::new(Vec::new()),
        }
    }

    /// Create a policy that only allows commands starting with the given prefixes.
    pub fn allow_only(prefixes: Vec<String>, rate_limit: Option<RateLimit>) -> Self {
        Self {
            allowed_prefixes: Some(prefixes.into_iter().collect()),
            rate_limit,
            command_times: Mutex::new(Vec::new()),
        }
    }

    /// Create a policy with only rate limiting.
    pub fn with_rate_limit(rate_limit: RateLimit) -> Self {
        Self {
            allowed_prefixes: None,
            rate_limit: Some(rate_limit),
            command_times: Mutex::new(Vec::new()),
        }
    }

    /// Check if a command is allowed under this policy.
    /// Returns Ok(()) if allowed, Err with reason if denied.
    pub fn check(&self, command: &str) -> Result<(), super::TransportError> {
        // Check command prefix allowlist
        if let Some(ref prefixes) = self.allowed_prefixes {
            let allowed = prefixes.iter().any(|p| command.starts_with(p.as_str()));
            if !allowed {
                return Err(super::TransportError::PolicyDenied(format!(
                    "command '{command}' does not match any allowed prefix"
                )));
            }
        }

        // Check rate limit
        if let Some(ref limit) = self.rate_limit {
            let mut times = self.command_times.lock().unwrap();
            let now = Instant::now();
            let window = std::time::Duration::from_millis(limit.window_ms);

            // Prune old entries
            times.retain(|&t| now.duration_since(t) < window);

            if times.len() >= limit.max_commands as usize {
                return Err(super::TransportError::RateLimited);
            }

            times.push(now);
        }

        Ok(())
    }
}

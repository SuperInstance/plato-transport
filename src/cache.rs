use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// A cached shadow value with associated metadata.
#[derive(Debug, Clone)]
pub struct CachedShadow {
    pub shadow: String,
    pub inserted_at: Instant,
    pub ttl: Duration,
}

impl CachedShadow {
    /// Returns true if this cache entry has expired.
    pub fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.inserted_at) > self.ttl
    }
}

/// Shadow cache keyed by `(sense_module, resource_id)` with per-entry TTL.
#[derive(Debug)]
pub struct ShadowCache {
    entries: Mutex<HashMap<(String, String), CachedShadow>>,
}

impl ShadowCache {
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
        }
    }

    /// Retrieve a cached shadow if it exists and hasn't expired.
    pub fn get(&self, module: &str, resource: &str) -> Option<CachedShadow> {
        let entries = self.entries.lock().unwrap();
        entries
            .get(&(module.to_string(), resource.to_string()))
            .filter(|c| !c.is_expired())
            .cloned()
    }

    /// Store a shadow value with the given TTL.
    pub fn put(&self, module: &str, resource: &str, shadow: String, ttl: Duration) {
        let mut entries = self.entries.lock().unwrap();
        entries.insert(
            (module.to_string(), resource.to_string()),
            CachedShadow {
                shadow,
                inserted_at: Instant::now(),
                ttl,
            },
        );
    }

    /// Invalidate a specific cache entry.
    pub fn invalidate(&self, module: &str, resource: &str) {
        let mut entries = self.entries.lock().unwrap();
        entries.remove(&(module.to_string(), resource.to_string()));
    }

    /// Remove all expired entries.
    #[allow(dead_code)]
    pub fn evict_expired(&self) {
        let mut entries = self.entries.lock().unwrap();
        entries.retain(|_, v| !v.is_expired());
    }
}

impl Default for ShadowCache {
    fn default() -> Self {
        Self::new()
    }
}

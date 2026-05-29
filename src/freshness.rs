/// Describes how fresh data from a sense module is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Freshness {
    /// Real-time / live data (in-process or direct connection).
    Hot,
    /// Periodically polled; `poll_interval_ms` is the typical latency.
    Warm { poll_interval_ms: u64 },
    /// Stale / snapshot data; `snapshot_age_ms` indicates how old it may be.
    Cold { snapshot_age_ms: u64 },
}

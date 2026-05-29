use std::fmt;

/// Errors that can occur during transport operations.
#[derive(Debug, Clone)]
pub enum TransportError {
    /// The transport is disconnected or was never connected.
    Disconnected,
    /// A receive operation timed out before a response arrived.
    Timeout,
    /// An I/O error occurred (e.g. socket closed, network failure).
    Io(String),
    /// The command was blocked by the transport policy.
    PolicyDenied(String),
    /// The rate limit was exceeded.
    RateLimited,
    /// Generic transport failure.
    Other(String),
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransportError::Disconnected => write!(f, "transport disconnected"),
            TransportError::Timeout => write!(f, "receive timed out"),
            TransportError::Io(msg) => write!(f, "I/O error: {msg}"),
            TransportError::PolicyDenied(msg) => write!(f, "policy denied: {msg}"),
            TransportError::RateLimited => write!(f, "rate limit exceeded"),
            TransportError::Other(msg) => write!(f, "transport error: {msg}"),
        }
    }
}

impl std::error::Error for TransportError {}

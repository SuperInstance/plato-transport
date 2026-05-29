use crate::{Freshness, TransportError};

pub mod in_process;
pub mod network;
pub mod unix;

/// Trait for sensory transport — how sense modules communicate.
pub trait SenseTransport: Send + Sync {
    /// Send a command string to the sense module.
    fn send(&self, command: &str) -> Result<(), TransportError>;
    /// Receive a shadow/response from the sense module, waiting up to `timeout_ms`.
    fn recv(&self, timeout_ms: u64) -> Result<String, TransportError>;
    /// How fresh is data from this transport?
    fn freshness(&self) -> Freshness;
    /// Is the transport currently connected?
    fn is_connected(&self) -> bool;
}

/// Factory for creating transports.
pub struct Transport;

impl Transport {
    /// Create an in-process (channel-based) transport.
    pub fn in_process() -> Box<dyn SenseTransport> {
        Box::new(in_process::InProcessTransport::new())
    }

    /// Create a Unix socket transport (simulated for testing).
    pub fn unix_socket(path: &str) -> Box<dyn SenseTransport> {
        Box::new(unix::UnixSocketTransport::new(path))
    }

    /// Create a TCP-based network transport (simulated for testing).
    pub fn tcp(addr: &str) -> Box<dyn SenseTransport> {
        Box::new(network::NetworkTransport::new(addr))
    }
}

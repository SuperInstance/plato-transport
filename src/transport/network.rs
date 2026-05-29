use std::sync::Mutex;

use crate::{Freshness, SenseTransport, TransportError};

/// TCP-based network transport for remote sense modules — simulated for testing.
pub struct NetworkTransport {
    address: String,
    outgoing: Mutex<Vec<String>>,
    incoming: Mutex<Vec<String>>,
    connected: Mutex<bool>,
}

impl NetworkTransport {
    pub fn new(addr: &str) -> Self {
        Self {
            address: addr.to_string(),
            outgoing: Mutex::new(Vec::new()),
            incoming: Mutex::new(Vec::new()),
            connected: Mutex::new(true),
        }
    }

    /// Simulate a response arriving (for testing).
    pub fn inject_response(&self, response: &str) {
        self.incoming.lock().unwrap().push(response.to_string());
    }

    /// Disconnect the transport.
    pub fn disconnect(&self) {
        *self.connected.lock().unwrap() = false;
    }

    /// Get the remote address.
    #[allow(dead_code)]
    pub fn address(&self) -> &str {
        &self.address
    }
}

impl SenseTransport for NetworkTransport {
    fn send(&self, command: &str) -> Result<(), TransportError> {
        if !self.is_connected() {
            return Err(TransportError::Disconnected);
        }
        self.outgoing.lock().unwrap().push(command.to_string());
        Ok(())
    }

    fn recv(&self, _timeout_ms: u64) -> Result<String, TransportError> {
        if !self.is_connected() {
            return Err(TransportError::Disconnected);
        }
        let mut incoming = self.incoming.lock().unwrap();
        if !incoming.is_empty() {
            return Ok(incoming.remove(0));
        }
        Err(TransportError::Timeout)
    }

    fn freshness(&self) -> Freshness {
        Freshness::Cold {
            snapshot_age_ms: 1000,
        }
    }

    fn is_connected(&self) -> bool {
        *self.connected.lock().unwrap()
    }
}

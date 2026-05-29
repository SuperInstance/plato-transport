use std::sync::Mutex;

use crate::{Freshness, SenseTransport, TransportError};

/// In-process transport using channels — zero-copy for embedded/single-process.
pub struct InProcessTransport {
    outgoing: Mutex<Vec<String>>,
    incoming: Mutex<Vec<String>>,
    connected: Mutex<bool>,
}

impl InProcessTransport {
    pub fn new() -> Self {
        Self {
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

    /// Read the last sent command (for testing).
    pub fn last_sent(&self) -> Option<String> {
        self.outgoing.lock().unwrap().last().cloned()
    }
}

impl Default for InProcessTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl SenseTransport for InProcessTransport {
    fn send(&self, command: &str) -> Result<(), TransportError> {
        if !self.is_connected() {
            return Err(TransportError::Disconnected);
        }
        self.outgoing.lock().unwrap().push(command.to_string());
        Ok(())
    }

    fn recv(&self, timeout_ms: u64) -> Result<String, TransportError> {
        if !self.is_connected() {
            return Err(TransportError::Disconnected);
        }
        // Check if we have data immediately
        {
            let mut incoming = self.incoming.lock().unwrap();
            if !incoming.is_empty() {
                return Ok(incoming.remove(0));
            }
        }
        // No data available — in a real impl we'd wait; here we simulate timeout
        if timeout_ms == 0 {
            return Err(TransportError::Timeout);
        }
        // For testing simplicity, we'll just return timeout if nothing is buffered
        // A real impl would use condvar to wait up to timeout_ms
        Err(TransportError::Timeout)
    }

    fn freshness(&self) -> Freshness {
        Freshness::Hot
    }

    fn is_connected(&self) -> bool {
        *self.connected.lock().unwrap()
    }
}

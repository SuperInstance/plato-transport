use std::sync::Mutex;

use crate::{Freshness, SenseTransport, TransportError};

/// Unix socket transport for local IPC — simulated for testing.
pub struct UnixSocketTransport {
    socket_path: String,
    outgoing: Mutex<Vec<String>>,
    incoming: Mutex<Vec<String>>,
    connected: Mutex<bool>,
}

impl UnixSocketTransport {
    pub fn new(path: &str) -> Self {
        Self {
            socket_path: path.to_string(),
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

    /// Get the socket path.
    #[allow(dead_code)]
    pub fn socket_path(&self) -> &str {
        &self.socket_path
    }
}

impl SenseTransport for UnixSocketTransport {
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
        Freshness::Warm {
            poll_interval_ms: 10,
        }
    }

    fn is_connected(&self) -> bool {
        *self.connected.lock().unwrap()
    }
}

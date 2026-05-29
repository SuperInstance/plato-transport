# plato-transport

Sensory transport abstraction for the Plato project — how sense modules communicate whether they're in the same process, on the same machine via Unix socket, or across the network via TCP.

## Features

- **`SenseTransport` trait** — unified interface for `send`, `recv`, `freshness`, and `is_connected`
- **`InProcessTransport`** — zero-copy channel-based transport for embedded/single-process use
- **`UnixSocketTransport`** — local IPC with credential passing (simulated for testing)
- **`NetworkTransport`** — TCP-based remote sense modules (simulated for testing)
- **`Freshness`** — `Hot` (real-time), `Warm { poll_interval_ms }`, `Cold { snapshot_age_ms }`
- **`ShadowCache`** — TTL-based cache keyed by `(sense_module, resource_id)`
- **`TransportPolicy`** — command allowlists and rate limiting

## Quick Start

```rust
use plato_transport::prelude::*;

// Create transports
let in_proc = Transport::in_process();
let unix = Transport::unix_socket("/tmp/sense.sock");
let tcp = Transport::tcp("192.168.1.100:9000");

// Use the ShadowCache
let cache = ShadowCache::new();
cache.put("vision", "camera_0", "{\"objects\": 5}".into(), std::time::Duration::from_secs(30));
let shadow = cache.get("vision", "camera_0");

// Enforce policy
let policy = TransportPolicy::allow_only(vec!["query:".into()], Some(RateLimit { max_commands: 100, window_ms: 1000 }));
policy.check("query:sensors").unwrap();
```

## License

MIT

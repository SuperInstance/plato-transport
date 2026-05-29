# plato-transport — Sense Transport Abstraction

Unified transport layer for sense modules — whether they're in the same process, on the same machine via Unix socket, or across the network via TCP. One trait, three implementations, zero friction.

**Part of the [Plato](https://github.com/SuperInstance/plato-shell) ecosystem.**

## What This Gives You

- **`SenseTransport` trait** — `send`, `recv`, `freshness`, `is_connected` for any transport
- **InProcessTransport** — zero-copy channels for embedded/single-process use
- **UnixSocketTransport** — local IPC with credential passing
- **NetworkTransport** — TCP-based remote sense modules
- **ShadowCache** — TTL-based cache keyed by `(sense_module, resource_id)`
- **TransportPolicy** — command allowlists and rate limiting
- **Freshness levels** — Hot (real-time), Warm (polling), Cold (snapshot)

## Quick Start

```rust
use plato_transport::prelude::*;

// Create transports
let in_proc = Transport::in_process();
let unix = Transport::unix_socket("/tmp/sense.sock");
let tcp = Transport::tcp("192.168.1.100:9000");

// Use the ShadowCache
let cache = ShadowCache::new();
cache.put("vision", "camera_0", "{\"objects\": 5}".into(), Duration::from_secs(30));
let shadow = cache.get("vision", "camera_0");

// Enforce policy
let policy = TransportPolicy::allow_only(vec!["query:".into()], Some(RateLimit { max_commands: 100, window_ms: 1000 }));
policy.check("query:sensors")?;
```

## How It Fits

Moves sense data between modules. [plato-vision](https://github.com/SuperInstance/plato-vision) and [plato-sonar-text](https://github.com/SuperInstance/plato-sonar-text) produce shadows; transport delivers them to [plato-correlator](https://github.com/SuperInstance/plato-correlator). Works with [plato-policy](https://github.com/SuperInstance/plato-policy) for access control.

## Testing

```bash
cargo test
```

## License

MIT

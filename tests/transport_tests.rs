use std::thread;
use std::time::Duration;

use plato_transport::*;
use plato_transport::in_process::InProcessTransport;
use plato_transport::unix::UnixSocketTransport;
use plato_transport::network::NetworkTransport;

// ── InProcess Transport ──────────────────────────────────────────────

#[test]
fn in_process_send_recv_works() {
    let ip = InProcessTransport::new();
    ip.inject_response("shadow:ok");
    ip.send("query:sensors").unwrap();
    assert_eq!(ip.last_sent(), Some("query:sensors".to_string()));
    let resp = ip.recv(100).unwrap();
    assert_eq!(resp, "shadow:ok");
}

// ── Unix Socket Transport ────────────────────────────────────────────

#[test]
fn unix_socket_send_recv_works() {
    let transport = UnixSocketTransport::new("/tmp/test.sock");
    transport.inject_response("unix:response");
    transport.send("cmd:read").unwrap();
    let resp = transport.recv(100).unwrap();
    assert_eq!(resp, "unix:response");
}

// ── TCP Network Transport ────────────────────────────────────────────

#[test]
fn tcp_transport_send_recv_works() {
    let transport = NetworkTransport::new("127.0.0.1:5000");
    transport.inject_response("tcp:data");
    transport.send("cmd:fetch").unwrap();
    let resp = transport.recv(100).unwrap();
    assert_eq!(resp, "tcp:data");
}

// ── Freshness Levels ─────────────────────────────────────────────────

#[test]
fn freshness_levels_correct_per_transport() {
    let ip = InProcessTransport::new();
    assert_eq!(ip.freshness(), Freshness::Hot);

    let unix = UnixSocketTransport::new("/tmp/test.sock");
    assert_eq!(
        unix.freshness(),
        Freshness::Warm {
            poll_interval_ms: 10
        }
    );

    let tcp = NetworkTransport::new("127.0.0.1:5000");
    assert_eq!(
        tcp.freshness(),
        Freshness::Cold {
            snapshot_age_ms: 1000
        }
    );
}

// ── Shadow Cache ─────────────────────────────────────────────────────

#[test]
fn shadow_cache_stores_and_retrieves() {
    let cache = ShadowCache::new();
    cache.put("vision", "camera_0", "{\"objects\": 5}".to_string(), Duration::from_secs(60));
    let cached = cache.get("vision", "camera_0").unwrap();
    assert_eq!(cached.shadow, "{\"objects\": 5}");
}

#[test]
fn shadow_cache_expires_by_ttl() {
    let cache = ShadowCache::new();
    cache.put("lidar", "scan_0", "points:1024".to_string(), Duration::from_millis(10));
    thread::sleep(Duration::from_millis(20));
    assert!(cache.get("lidar", "scan_0").is_none());
}

#[test]
fn shadow_cache_invalidation_works() {
    let cache = ShadowCache::new();
    cache.put("audio", "mic_0", "freq:440".to_string(), Duration::from_secs(60));
    assert!(cache.get("audio", "mic_0").is_some());
    cache.invalidate("audio", "mic_0");
    assert!(cache.get("audio", "mic_0").is_none());
}

// ── Timeout ──────────────────────────────────────────────────────────

#[test]
fn recv_timeout_returns_error() {
    let transport = InProcessTransport::new();
    let result = transport.recv(0);
    assert!(matches!(result, Err(TransportError::Timeout)));
}

// ── Disconnected ─────────────────────────────────────────────────────

#[test]
fn disconnected_transport_returns_error() {
    let transport = InProcessTransport::new();
    transport.disconnect();
    assert!(!transport.is_connected());

    let result = transport.send("cmd");
    assert!(matches!(result, Err(TransportError::Disconnected)));

    let result = transport.recv(100);
    assert!(matches!(result, Err(TransportError::Disconnected)));
}

// ── Policy: unauthorized commands ────────────────────────────────────

#[test]
fn policy_blocks_unauthorized_commands() {
    let policy = TransportPolicy::allow_only(
        vec!["query:".to_string(), "read:".to_string()],
        None,
    );
    assert!(policy.check("query:sensors").is_ok());
    assert!(policy.check("read:camera").is_ok());
    let result = policy.check("delete:all");
    assert!(matches!(result, Err(TransportError::PolicyDenied(_))));
}

// ── Policy: rate limits ──────────────────────────────────────────────

#[test]
fn policy_enforces_rate_limits() {
    let policy = TransportPolicy::with_rate_limit(RateLimit {
        max_commands: 3,
        window_ms: 1000,
    });

    assert!(policy.check("cmd:a").is_ok());
    assert!(policy.check("cmd:b").is_ok());
    assert!(policy.check("cmd:c").is_ok());
    let result = policy.check("cmd:d");
    assert!(matches!(result, Err(TransportError::RateLimited)));
}

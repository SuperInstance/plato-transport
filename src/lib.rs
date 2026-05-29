mod cache;
mod error;
mod freshness;
mod policy;
mod transport;

pub use cache::{CachedShadow, ShadowCache};
pub use error::TransportError;
pub use freshness::Freshness;
pub use policy::{RateLimit, TransportPolicy};
pub use transport::{
    in_process::InProcessTransport, network::NetworkTransport, unix::UnixSocketTransport,
    SenseTransport, Transport,
};

pub mod in_process { pub use crate::transport::in_process::InProcessTransport; }
pub mod unix { pub use crate::transport::unix::UnixSocketTransport; }
pub mod network { pub use crate::transport::network::NetworkTransport; }

pub mod prelude {
    pub use crate::{
        CachedShadow, Freshness, InProcessTransport, NetworkTransport, RateLimit, SenseTransport,
        ShadowCache, Transport, TransportError, TransportPolicy, UnixSocketTransport,
    };
}

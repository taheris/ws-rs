//! The util module rexports some tools from mio in order to facilitate handling timeouts.

/// Used to identify some timed-out event.
pub use mio::Token;
/// A handle to a specific timeout.
pub use mio::Timeout;

//! # Axor
//!
//! Axor is a zero-overhead, agent-based backend framework for Rust.
//!
//! It provides:
//! - Strongly-typed agents with auto-injection
//! - Simple operation declarations with `#[operation]`
//! - Optional RPC-style invocation via `Payload`
//!
//! Use `axor::prelude::*` for a complete developer-friendly import.


mod agent;
mod context;
mod operation;
mod inject;
mod payload;

pub use agent::*;
pub use context::*;
pub use operation::*;
pub use inject::*;
pub use payload::*;

/// Auto-imports all the commonly used types and macros for agent development.
#[doc(hidden)]
pub mod prelude;

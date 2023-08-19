//! A minimal (i.e. very incomplete) implementation of a Redis server and
//! client based on rpc.
//!
//! The purpose of this project is to provide a usage example of a
//! Rust RPC project built with Volo.

#![feature(impl_trait_in_assoc_type)]

pub mod gen {
    volo::include_service!("volo_gen.rs");
}
mod db;
mod server;
pub use server::Server;

/// Default port that a redis server listens on.
///
/// Used if no port is specified.
pub const DEFAULT_PORT: u16 = 6379;

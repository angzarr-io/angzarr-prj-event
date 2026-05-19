#![allow(clippy::result_large_err)]

//! Event storage projector for Angzarr.
//!
//! Writes every event as JSON to a SQL table (SQLite or Postgres), keyed by
//! `(domain, root_id, sequence)`, for ad-hoc querying and debugging.

pub mod event;

pub use event::{connect_pool, EventService, EventServiceHandle, Pool};

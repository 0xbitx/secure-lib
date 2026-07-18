//! # secure-lib
//!
//! A security-focused utility library providing authentication,
//! cryptographic hashing, and configuration management primitives.
//!
//! ## Features
//!
//! - Password hashing with Argon2
//! - JWT token creation and validation
//! - Secure configuration loading from environment
//! - Audit logging utilities

pub mod auth;
pub mod config;
pub mod hashing;
pub mod logging;

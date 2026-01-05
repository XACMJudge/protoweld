//! Operating system abstraction layer for cross-platform operations.
//!
//! This module provides platform-independent interfaces for file operations,
//! command execution, and system interactions. Currently supports Unix-like
//! systems (Linux, macOS) with plans for Windows support.

pub mod types;
pub mod unix_manager;
pub mod shared;

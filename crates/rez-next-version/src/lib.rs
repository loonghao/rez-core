//! # Rez Core Version
//!
//! Version parsing, comparison, and range handling for Rez Core.
//!
//! This crate provides:
//! - Version parsing and validation
//! - Version comparison and ordering
//! - Version range operations
//! - Token-based version representation
//! - Python bindings for version operations

#[cfg(feature = "python-bindings")]
use pyo3::prelude::*;
use rez_next_common::RezCoreError;

pub mod parser;
pub mod range; // Always available for benchmarks and core functionality
#[cfg(feature = "python-bindings")]
pub mod token;
pub mod version;
#[cfg(feature = "python-bindings")]
pub mod version_token;
// pub mod optimized_parser;

// Re-export main types
pub use parser::{StateMachineParser, VersionParser};
// Always export VersionRange as it's needed by benchmarks and other core functionality
pub use range::VersionRange;
#[cfg(feature = "python-bindings")]
pub use token::PyVersionToken;
pub use version::Version;
#[cfg(feature = "python-bindings")]
pub use version_token::{AlphanumericVersionToken, NumericToken, VersionToken};
// pub use optimized_parser::{OptimizedVersionParser, BatchVersionParser, ParsedVersionComponents};

// Define a custom error type for version parsing
#[derive(Debug)]
pub struct VersionParseError(pub String);

impl std::fmt::Display for VersionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Version parse error: {}", self.0)
    }
}

impl std::error::Error for VersionParseError {}

impl From<RezCoreError> for VersionParseError {
    fn from(err: RezCoreError) -> Self {
        VersionParseError(err.to_string())
    }
}

// Make it a Python exception
#[cfg(feature = "python-bindings")]
pyo3::create_exception!(rez_core, PyVersionParseError, pyo3::exceptions::PyException);

/// Parse a version string into a Version object
#[cfg(feature = "python-bindings")]
#[pyfunction]
pub fn parse_version(version_str: &str) -> PyResult<Version> {
    Version::parse(version_str).map_err(|e| PyErr::new::<PyVersionParseError, _>(e.to_string()))
}

/// Parse a version range string into a VersionRange object
#[cfg(feature = "python-bindings")]
#[pyfunction]
pub fn parse_version_range(range_str: &str) -> PyResult<VersionRange> {
    VersionRange::parse(range_str)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{:?}", e)))
}

/// Python module for rez_core.version
#[cfg(feature = "python-bindings")]
#[pymodule]
pub fn version_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Version system
    m.add_class::<Version>()?;
    m.add_class::<VersionRange>()?;
    m.add_class::<PyVersionToken>()?;

    // Version tokens (rez-compatible)
    m.add_class::<VersionToken>()?;
    m.add_class::<NumericToken>()?;
    m.add_class::<AlphanumericVersionToken>()?;

    // Version parsing functions
    m.add_function(wrap_pyfunction!(parse_version, m)?)?;
    m.add_function(wrap_pyfunction!(parse_version_range, m)?)?;

    // Error types
    m.add(
        "PyVersionParseError",
        m.py().get_type::<PyVersionParseError>(),
    )?;

    Ok(())
}

#[cfg(test)]
mod tests;

//! `Validator` — port contract for configuration validation.

/// Validates a consumer configuration value before use.
pub trait Validator {
    /// Returns `Ok(())` when valid, or a human-readable error string.
    fn validate(&self) -> Result<(), String>;
}

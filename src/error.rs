//! Error types for ui-test-rs

use thiserror::Error;

/// Main error type for ui-test-rs
#[derive(Debug, Error)]
pub enum UiTestError {
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Test discovery failed
    #[error("Test discovery failed: {0}")]
    Discovery(String),

    /// Playwright MCP connection failed
    #[error("Playwright MCP connection failed: {0}")]
    PlaywrightConnection(String),

    /// Browser action failed
    #[error("Browser action failed: {0}")]
    BrowserAction(String),

    /// Test assertion failed
    #[error("Assertion failed: {0}")]
    Assertion(String),

    /// Test timeout
    #[error("Test timeout after {0:?}")]
    Timeout(std::time::Duration),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type alias using UiTestError
pub type Result<T> = std::result::Result<T, UiTestError>;

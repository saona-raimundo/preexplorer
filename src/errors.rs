//! Errors from the crate.

use thiserror::Error;

/// Error from writting while saving files (data or plot scripts).
#[derive(Error, Debug)]
pub enum PreexplorerError {
    #[error("Saving error.")]
    Saving(#[from] std::io::Error),
    #[error("Plotting error.")]
    Plotting(std::io::Error),
}

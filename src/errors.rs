use thiserror::Error;

/// Error from writting while saving files (data or plot scripts).
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum PreexplorerError {
    #[error("Saving error.")]
    Saving(#[from] std::io::Error),
    #[error("Plotting error.")]
    Plotting(#[source] std::io::Error),
    #[error("Removing error: {1}")]
    Removing(#[source] std::io::Error, String),
}

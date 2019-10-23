use failure::Fail;

#[derive(Fail, Debug)]
#[fail(display = "Failed while saving or plotting.")]
pub struct SavingError {
    #[cause]
    pub(crate) cause: std::io::Error,
}

impl SavingError {
    pub fn new(e: std::io::Error) -> Self {
        SavingError { cause: e }
    }
}

impl From<std::io::Error> for SavingError {
    fn from(e: std::io::Error) -> Self {
        SavingError { cause: e }
    }
}
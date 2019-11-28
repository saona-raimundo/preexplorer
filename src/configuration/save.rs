#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub(crate) struct SaveConfiguration {
    extension: String,
}

impl SaveConfiguration {
    pub(crate) fn default() -> SaveConfiguration {
        SaveConfiguration {
            extension: String::from(".txt"),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub(crate) struct SaveConfiguration {
    extension: String,
    header: bool,
}

impl SaveConfiguration {
    pub(crate) fn default() -> SaveConfiguration {
        let extension = String::from("txt");
        let header = true;

        SaveConfiguration {
            extension,
            header,
        }
    }

    pub(crate) fn extension(&mut self, extension: String) -> &mut Self {
        self.extension = extension;
        self
    }
    pub(crate) fn header(&mut self, header: bool) -> &mut Self {
        self.header = header;
        self
    }


    pub(crate) fn get_extension(&self) -> &str {
        &self.extension
    }
    pub(crate) fn get_header(&self) -> bool {
        self.header
    }
}

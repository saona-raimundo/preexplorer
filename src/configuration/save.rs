use crate::constants::DATA_DIR;
use std::ffi::OsStr;
use std::path::PathBuf;

// Traits
use getset::{Getters};

#[derive(Getters, Debug, PartialOrd, PartialEq, Clone)]
#[getset(get = "pub")]
pub(crate) struct SaveConfiguration {
    path_buf: PathBuf,
    header: bool,
    date: chrono::DateTime<chrono::Local>,
    id: Option<String>,
}

impl SaveConfiguration {
    pub(crate) fn set_extension<S: AsRef<OsStr>>(&mut self, extension: S) -> &mut Self {
        self.path_buf.set_extension(extension);
        self
    }
    pub(crate) fn set_header(&mut self, header: bool) -> &mut Self {
        self.header = header;
        self
    }
    pub(crate) fn set_date(&mut self, date: chrono::DateTime<chrono::Local>) -> &mut Self {
        self.date = date;
        self
    }
    pub(crate) fn set_id(&mut self, id: String) -> &mut Self {
        if let Some(extension) = self.path_buf.clone().extension() {
            self.path_buf.set_file_name(&id);
            self.path_buf.set_extension(extension);
        } else {
            self.path_buf.set_file_name(&id);
        }
        self.id = Some(id);

        self
    }

    pub(crate) fn extension(&self) -> Option<&OsStr> {
        self.path_buf.extension()
    }
    pub(crate) fn checked_id(&self) -> &String {
        match &self.id {
            Some(id) => id,
            None => panic!("Uninitialized id. Consider giving an id before processing."),
        }
    }
}

impl Default for SaveConfiguration {
    fn default() -> Self {
        let mut path_buf: PathBuf = DATA_DIR.iter().collect();
        path_buf.push("none");
        path_buf.set_extension("txt");
        let header = true;
        let date = chrono::Local::now();
        let id = None;

        SaveConfiguration {
            path_buf,
            header,
            date,
            id,
        }
    }
}

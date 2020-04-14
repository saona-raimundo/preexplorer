use crate::constants::DATA_DIR;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub(crate) struct SaveConfiguration {
    path_buf: PathBuf,
    header: bool,
    date: chrono::DateTime<chrono::Local>,
    id: Option<String>,
}

impl SaveConfiguration {
    pub(crate) fn extension<S: AsRef<OsStr>>(&mut self, extension: S) -> &mut Self {
        self.path_buf.set_extension(extension);
        self
    }
    pub(crate) fn header(&mut self, header: bool) -> &mut Self {
        self.header = header;
        self
    }
    pub(crate) fn date(&mut self, date: chrono::DateTime<chrono::Local>) -> &mut Self {
        self.date = date;
        self
    }
    pub(crate) fn id(&mut self, id: String) -> &mut Self {
        if let Some(extension) = self.path_buf.clone().extension() {
            self.path_buf.set_file_name(&id);
            self.path_buf.set_extension(extension);
        } else {
            self.path_buf.set_file_name(&id);
        }
        self.id = Some(id);

        self
    }

    pub(crate) fn get_extension(&self) -> Option<&OsStr> {
        self.path_buf.extension()
    }
    pub(crate) fn get_path(&self) -> &Path {
        self.path_buf.as_path()
    }
    pub(crate) fn get_header(&self) -> bool {
        self.header
    }
    pub(crate) fn get_date(&self) -> &chrono::DateTime<chrono::Local> {
        &self.date
    }
    pub(crate) fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }
    pub(crate) fn get_checked_id(&self) -> &String {
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

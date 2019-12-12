#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub(crate) struct SaveConfiguration {
    extension: String,
    header: bool,
    date: chrono::DateTime<chrono::Local>,
    id: Option<String>,  
}

impl SaveConfiguration {
    pub(crate) fn extension(&mut self, extension: String) -> &mut Self {
        self.extension = extension;
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
        self.id = Some(id);
        self
    }
    

    pub(crate) fn get_extension(&self) -> &str {
        &self.extension
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
        let extension = String::from("txt");
        let header = true;
        let date = chrono::Local::now();
        let id = None;

        SaveConfiguration { extension, header, date, id }
    }
}
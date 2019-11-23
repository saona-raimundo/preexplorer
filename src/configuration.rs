#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub(crate) struct Configuration {
    save_config: SaveConfiguration,
    plot_config: PlotConfiguration,
}

impl Configuration {
    pub(crate) fn default() -> Configuration {
        let save_config = crate::configuration::SaveConfiguration::default();
        let plot_config = crate::configuration::PlotConfiguration::default();

        Configuration {
            save_config,
            plot_config,
        }
    }

    pub(crate) fn set_title(&mut self, title: String) -> &mut Self {
        self.plot_config.set_title(title);
        self
    }
    pub(crate) fn set_logx(&mut self, logx: f64) -> &mut Self {
        self.plot_config.set_logx(logx);
        self
    }
    pub(crate) fn set_logy(&mut self, logy: f64) -> &mut Self {
        self.plot_config.set_logy(logy);
        self
    }

    pub(crate) fn title(&self) -> Option<String> {
        self.plot_config.title()
    }
    pub(crate) fn logx(&self) -> Option<f64> {
        self.plot_config.logx()
    }
    pub(crate) fn logy(&self) -> Option<f64> {
        self.plot_config.logy()
    }
}

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

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub(crate) struct PlotConfiguration {
    title: Option<String>,
    logx: Option<f64>,
    logy: Option<f64>,
}

impl PlotConfiguration {
    pub(crate) fn default() -> PlotConfiguration {
        let title = None;
        let logx = None;
        let logy = None;

        PlotConfiguration { title, logx, logy }
    }

    pub(crate) fn set_title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }
    pub(crate) fn set_logx(&mut self, logx: f64) -> &mut Self {
        self.logx = Some(logx);
        self
    }
    pub(crate) fn set_logy(&mut self, logy: f64) -> &mut Self {
        self.logy = Some(logy);
        self
    }

    pub(crate) fn title(&self) -> Option<String> {
        match &self.title {
            Some(title) => Some(title.to_string()),
            None => None,
        }
    }
    pub(crate) fn logx(&self) -> Option<f64> {
        self.logx
    }
    pub(crate) fn logy(&self) -> Option<f64> {
        self.logy
    }
}

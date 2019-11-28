

mod save;
mod plot;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Configuration {
    save_config: crate::configuration::save::SaveConfiguration,
    plot_config: crate::configuration::plot::PlotConfiguration,
}

impl Configuration {
    pub(crate) fn default() -> Configuration {
        let save_config = crate::configuration::save::SaveConfiguration::default();
        let plot_config = crate::configuration::plot::PlotConfiguration::default();

        Configuration {
            save_config,
            plot_config,
        }
    }

    pub(crate) fn base_plot_script(&self) -> String {
        self.plot_config.base_plot_script()
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
    pub(crate) fn set_labelx(&mut self, labelx: String) -> &mut Self {
        self.plot_config.set_labelx(labelx);
        self
    }
    pub(crate) fn set_labely(&mut self, labely: String) -> &mut Self {
        self.plot_config.set_labely(labely);
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
    pub(crate) fn labelx(&self) -> Option<String> {
        self.plot_config.labelx()
    }
    pub(crate) fn labely(&self) -> Option<String> {
        self.plot_config.labely()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_set_logx() {
        let mut config = Configuration::default();

        assert_eq!(config.logx(), None);

        config.set_logx(10.);

        assert_eq!(config.logx(), Some(10.));
    }

    #[test]
    fn check_set_logy() {
        let mut config = Configuration::default();

        assert_eq!(config.logx(), None);

        config.set_logy(10.);

        assert_eq!(config.logy(), Some(10.));
    }

    #[test]
    fn check_set_labelx() {
        let mut config = Configuration::default();

        assert_eq!(config.labelx(), None);
        
        config.set_labelx(String::from("try"));

        assert_eq!(config.labelx(), Some(String::from("try")));
    }

    #[test]
    fn check_set_labely() {
        let mut config = Configuration::default();

        assert_eq!(config.labely(), None);
        
        config.set_labely(String::from("try"));

        assert_eq!(config.labely(), Some(String::from("try")));
    }
}
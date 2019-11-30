

pub mod save;
pub mod plot;

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

    pub(crate) fn base_plot_script_comparison(&self) -> String {
        self.plot_config.base_plot_script_comparison()
    }

    /////////////////////////// PlotConfiguration
    // Setting
    pub(crate) fn title(&mut self, title: String) -> &mut Self {
        self.plot_config.title(title);
        self
    }
    pub(crate) fn logx(&mut self, logx: f64) -> &mut Self {
        self.plot_config.logx(logx);
        self
    }
    pub(crate) fn logy(&mut self, logy: f64) -> &mut Self {
        self.plot_config.logy(logy);
        self
    }
    pub(crate) fn labelx(&mut self, labelx: String) -> &mut Self {
        self.plot_config.labelx(labelx);
        self
    }
    pub(crate) fn labely(&mut self, labely: String) -> &mut Self {
        self.plot_config.labely(labely);
        self
    }
    pub(crate) fn style(&mut self, style: crate::configuration::plot::style::Style) -> &mut Self {
        self.plot_config.style(style);
        self
    }
    pub(crate) fn dashtype(&mut self, dashtype: usize) -> &mut Self {
        self.plot_config.dashtype(dashtype);
        self
    }

    // Getting
    pub(crate) fn get_title(&self) -> Option<&String> {
        self.plot_config.get_title()
    }
    pub(crate) fn get_logx(&self) -> Option<f64> {
        self.plot_config.get_logx()
    }
    pub(crate) fn get_logy(&self) -> Option<f64> {
        self.plot_config.get_logy()
    }
    pub(crate) fn get_labelx(&self) -> Option<&String> {
        self.plot_config.get_labelx()
    }
    pub(crate) fn get_labely(&self) -> Option<&String> {
        self.plot_config.get_labely()
    }
    pub(crate) fn get_style(&self) -> &crate::configuration::plot::style::Style {
        self.plot_config.get_style()
    }
    pub(crate) fn get_dashtype(&self) -> Option<usize> {
        self.plot_config.get_dashtype()
    }

    ////////// SaveConfiguration /////////////////
    // Setting
    pub(crate) fn extension(&mut self, extension: String) -> &mut Self {
        self.save_config.extension(extension);
        self
    }
    pub(crate) fn header(&mut self, header: bool) -> &mut Self {
        self.save_config.header(header);
        self
    }

    // Getting
    pub(crate) fn get_extension(&self) -> &str {
        self.save_config.get_extension()
    }
    pub(crate) fn get_header(&self) -> bool {
        self.save_config.get_header()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_logx() {
        let mut config = Configuration::default();

        assert_eq!(config.get_logx(), None);

        config.logx(10.);

        assert_eq!(config.get_logx(), Some(10.));
    }

    #[test]
    fn check_set_logy() {
        let mut config = Configuration::default();

        assert_eq!(config.get_logx(), None);

        config.logy(10.);

        assert_eq!(config.get_logy(), Some(10.));
    }

    #[test]
    fn check_set_labelx() {
        let mut config = Configuration::default();

        assert_eq!(config.get_labelx(), None);
        
        let labelx = String::from("try");
        config.labelx(labelx.clone());

        assert_eq!(config.get_labelx(), Some(&labelx));
    }

    #[test]
    fn check_set_labely() {
        let mut config = Configuration::default();

        assert_eq!(config.get_labely(), None);
        
        let labely = String::from("try");
        config.labely(labely.clone());

        assert_eq!(config.get_labely(), Some(&labely));
    }
}
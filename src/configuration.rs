use std::collections::HashMap;

pub mod plot;
pub mod save;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Configuration {
    save_config: crate::configuration::save::SaveConfiguration,
    plot_config: crate::configuration::plot::PlotConfiguration,
    custom_config: HashMap<String, String>,
}

impl Configuration {
    pub(crate) fn opening_plot_script(&self) -> String {
        self.plot_config.opening_plot_script()
    }

    pub(crate) fn opening_plot_script_comparison(&self) -> String {
        self.plot_config.opening_plot_script_comparison()
    }

    pub(crate) fn ending_plot_script(&self) -> String {
        self.plot_config.ending_plot_script()
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
    pub(crate) fn rangex(&mut self, rangex: (f64, f64)) -> &mut Self {
        self.plot_config.rangex(rangex);
        self
    }
    pub(crate) fn rangey(&mut self, rangey: (f64, f64)) -> &mut Self {
        self.plot_config.rangey(rangey);
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
    pub(crate) fn ticsx<T>(&mut self, ticsx: T) -> &mut Self 
    where
        T: Into<Option<String>>,
    {
        self.plot_config.ticsx(ticsx);
        self
    }
    pub(crate) fn ticsy<T>(&mut self, ticsy: T) -> &mut Self 
    where
        T: Into<Option<String>>,
    {
        self.plot_config.ticsy(ticsy);
        self
    }
    pub(crate) fn pause<T>(&mut self, pause: T) -> &mut Self 
    where
        T: Into<Option<f64>>,
    {
        self.plot_config.pause(pause);
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
    pub(crate) fn get_rangex(&self) -> Option<(f64, f64)> {
        self.plot_config.get_rangex()
    }
    pub(crate) fn get_rangey(&self) -> Option<(f64, f64)> {
        self.plot_config.get_rangey()
    }
    pub(crate) fn get_style(&self) -> &crate::configuration::plot::style::Style {
        self.plot_config.get_style()
    }
    pub(crate) fn get_dashtype(&self) -> Option<usize> {
        self.plot_config.get_dashtype()
    }
    pub(crate) fn get_ticsx(&self) -> Option<&String> 
    {
        self.plot_config.get_ticsx()
    }
    pub(crate) fn get_ticsy(&self) -> Option<&String> 
    {
        self.plot_config.get_ticsy()
    }
    pub(crate) fn get_pause(&self) -> Option<f64> 
    {
        self.plot_config.get_pause()
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
    pub(crate) fn date(&mut self, date: chrono::DateTime<chrono::Local>) -> &mut Self {
        self.save_config.date(date);
        self
    }
    pub(crate) fn id(&mut self, id: String) -> &mut Self {
        self.save_config.id(id);
        self
    }

    // Getting
    pub(crate) fn get_extension(&self) -> &str {
        self.save_config.get_extension()
    }
    pub(crate) fn get_header(&self) -> bool {
        self.save_config.get_header()
    }
    pub(crate) fn get_date(&self) -> &chrono::DateTime<chrono::Local> {
        self.save_config.get_date()
    }
    pub(crate) fn get_id(&self) -> Option<&String> {
        self.save_config.get_id()
    }
    pub(crate) fn get_checked_id(&self) -> &String {
        self.save_config.get_checked_id()
    }

    ////////////////// CustomConfiguration ///////////////////
    pub(crate) fn custom(&mut self, key: String, value: String) -> &mut Self {
    	self.custom_config.insert(key, value);
    	self
    }

    pub(crate) fn get_custom(&self, key: String) -> Option<&String> {
    	self.custom_config.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_id() {
        let mut config = Configuration::default();
        
        config.id(1.to_string());
        assert_eq!(config.get_id(), Some(&1.to_string()));

        config.id("1".to_string());
        assert_eq!(config.get_id(), Some(&String::from("1")));
    }

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

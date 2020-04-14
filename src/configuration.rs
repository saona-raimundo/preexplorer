//! Configuration for all basic options included.
//!
//! # Remarks
//!
//! See ``Configurable`` documentation for its main use.

// Traits
use core::fmt::Display;

// Structs
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;

pub mod plot;
pub mod save;

/// Configuration for all basic options included.
///
/// See the documentation of ``Configurable`` trait for all methods.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Configuration {
    save_config: crate::configuration::save::SaveConfiguration,
    plot_config: crate::configuration::plot::PlotConfiguration,
    custom_config: HashMap<String, String>,
}

impl Configuration {

    /// Opening for a plot script including all common or setted configurations. 
    pub fn opening_plot_script(&self) -> String {
        self.plot_config.opening_plot_script()
    }

    pub(crate) fn opening_plot_script_comparison(&self) -> String {
        self.plot_config.opening_plot_script_comparison()
    }

    /// Ending for a plot script including all common or setted configurations. 
    pub fn ending_plot_script(&self) -> String {
        self.plot_config.ending_plot_script()
    }
}

impl crate::Configurable for Configuration {
    fn configuration(&mut self) -> &mut Configuration {
        self
    }

    fn configuration_as_ref(&self) -> &Configuration {
        self
    }

    /////////////////////////// PlotConfiguration
    // Setting
    fn plot_extension<S: AsRef<OsStr>>(&mut self, extension: S) -> &mut Self {
        self.plot_config.extension(extension);
        self
    }
    fn title<S: Display>(&mut self, title: S) -> &mut Self {
        self.plot_config.title(title.to_string());
        self
    }
    fn logx<N: Into<f64>>(&mut self, logx: N) -> &mut Self {
        self.plot_config.logx(logx.into());
        self
    }
    fn logy<N: Into<f64>>(&mut self, logy: N) -> &mut Self {
        self.plot_config.logy(logy.into());
        self
    }
    fn labelx<S: Display>(&mut self, labelx: S) -> &mut Self {
        self.plot_config.labelx(labelx.to_string());
        self
    }
    fn labely<S: Display>(&mut self, labely: S) -> &mut Self {
        self.plot_config.labely(labely.to_string());
        self
    }
    fn rangex<S, T>(&mut self, left: S, right: T) -> &mut Self
    where
        f64: From<S>,
        f64: From<T>,
    {
        self.plot_config.rangex((f64::from(left), f64::from(right)));
        self
    }
    fn rangey<S, T>(&mut self, down: S, up: T) -> &mut Self
    where
        f64: From<S>,
        f64: From<T>,
    {
        self.plot_config.rangey((f64::from(down), f64::from(up)));
        self
    }
    fn style<S>(&mut self, style: S) -> &mut Self
    where
        crate::configuration::plot::style::Style: From<S>,
    {
        self.plot_config.style(style.into());
        self
    }
    fn dashtype(&mut self, dashtype: usize) -> &mut Self {
        self.plot_config.dashtype(dashtype);
        self
    }
    fn ticsx<T, S>(&mut self, ticsx: T) -> &mut Self
    where
        T: Into<Option<S>>,
        S: Display,
    {
        let ticsx: Option<S> = ticsx.into();
        self.plot_config.ticsx(ticsx.map(|t| t.to_string()));
        self
    }
    fn ticsy<T, S>(&mut self, ticsy: T) -> &mut Self
    where
        T: Into<Option<S>>,
        S: Display,
    {
        let ticsy: Option<S> = ticsy.into();
        self.plot_config.ticsy(ticsy.map(|t| t.to_string()));
        self
    }
    fn pause<T, S>(&mut self, pause: T) -> &mut Self
    where
        T: Into<Option<S>>,
        f64: From<S>,
    {
        let pause: Option<S> = pause.into();
        self.plot_config.pause(pause.map(f64::from));
        self
    }

    // Getting
    fn get_plot_extension(&self) -> Option<&OsStr> {
        self.plot_config.get_extension()
    }
    fn get_plot_path(&self) -> &Path {
        self.plot_config.get_path()
    }
    fn get_title(&self) -> Option<&String> {
        self.plot_config.get_title()
    }
    fn get_logx(&self) -> Option<f64> {
        self.plot_config.get_logx()
    }
    fn get_logy(&self) -> Option<f64> {
        self.plot_config.get_logy()
    }
    fn get_labelx(&self) -> Option<&String> {
        self.plot_config.get_labelx()
    }
    fn get_labely(&self) -> Option<&String> {
        self.plot_config.get_labely()
    }
    fn get_rangex(&self) -> Option<(f64, f64)> {
        self.plot_config.get_rangex()
    }
    fn get_rangey(&self) -> Option<(f64, f64)> {
        self.plot_config.get_rangey()
    }
    fn get_style(&self) -> &crate::configuration::plot::style::Style {
        self.plot_config.get_style()
    }
    fn get_dashtype(&self) -> Option<usize> {
        self.plot_config.get_dashtype()
    }
    fn get_ticsx(&self) -> Option<&String> {
        self.plot_config.get_ticsx()
    }
    fn get_ticsy(&self) -> Option<&String> {
        self.plot_config.get_ticsy()
    }
    fn get_pause(&self) -> Option<f64> {
        self.plot_config.get_pause()
    }

    ////////// SaveConfiguration /////////////////
    // Setting
    fn data_extension<S: AsRef<OsStr>>(&mut self, extension: S) -> &mut Self {
        self.save_config.extension(extension);
        self
    }
    fn header(&mut self, header: bool) -> &mut Self {
        self.save_config.header(header);
        self
    }
    fn date(&mut self, date: chrono::DateTime<chrono::Local>) -> &mut Self {
        self.save_config.date(date);
        self
    }
    fn id<S: Display>(&mut self, id: S) -> &mut Self {
        let id = id.to_string();
        self.plot_config.id(&id);
        self.save_config.id(id);
        self
    }

    // Getting
    fn get_data_extension(&self) -> Option<&OsStr> {
        self.save_config.get_extension()
    }
    fn get_data_path(&self) -> &Path {
        self.save_config.get_path()
    }
    fn get_header(&self) -> bool {
        self.save_config.get_header()
    }
    fn get_date(&self) -> &chrono::DateTime<chrono::Local> {
        self.save_config.get_date()
    }
    fn get_id(&self) -> Option<&String> {
        self.save_config.get_id()
    }
    fn get_checked_id(&self) -> &String {
        self.save_config.get_checked_id()
    }

    ////////////////// CustomConfiguration ///////////////////
    fn custom<S: Display, T: Display>(&mut self, key: S, value: T) -> &mut Self {
        self.custom_config
            .insert(key.to_string(), value.to_string());
        self
    }

    fn get_custom<S: Display>(&self, key: S) -> Option<&String> {
        self.custom_config.get(&key.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

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

    #[test]
    fn check_paths() {
        let mut config = Configuration::default();

        assert_eq!(config.get_id(), None);
        assert_eq!(config.get_data_extension().unwrap().to_str(), Some("txt"));

        assert_eq!(
            config.get_data_path().file_name().unwrap().to_str(),
            Some("none.txt")
        );
        assert_eq!(
            config.get_data_path().file_stem().unwrap().to_str(),
            Some("none")
        );
        assert_eq!(
            config.get_data_path().extension().unwrap().to_str(),
            Some("txt")
        );

        assert_eq!(
            config.get_plot_path().file_name().unwrap().to_str(),
            Some("none.gnu")
        );
        assert_eq!(
            config.get_data_path().file_stem().unwrap().to_str(),
            Some("none")
        );
        assert_eq!(
            config.get_plot_path().extension().unwrap().to_str(),
            Some("gnu")
        );

        config.id("testing".to_string());

        assert_eq!(
            config.get_plot_path().file_name().unwrap().to_str(),
            Some("testing.gnu")
        );
        assert_eq!(
            config.get_data_path().file_name().unwrap().to_str(),
            Some("testing.txt")
        );
    }
}

// Traits
use core::convert::TryInto;
use core::fmt::Debug;
use core::fmt::Display;

// Structs
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;

pub mod plot;
pub mod save;

pub use plot::{style::Style, *};

/// Configuration for all basic options included.
///
/// See trait [Configurable] documentation for its main use.
///
/// [Configurable]: trait.Configurable.html
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

impl crate::traits::Configurable for Configuration {
    fn configuration_mut(&mut self) -> &mut Configuration {
        self
    }

    fn configuration(&self) -> &Configuration {
        self
    }

    /////////////////////////// PlotConfiguration
    // Setting
    fn set_plot_extension<S: AsRef<OsStr>>(&mut self, extension: S) -> &mut Self {
        self.plot_config.set_extension(extension);
        self
    }
    fn set_title<S: Display>(&mut self, title: S) -> &mut Self {
        self.plot_config.set_title(title.to_string());
        self
    }
    fn set_logx<N: Into<f64>>(&mut self, logx: N) -> &mut Self {
        self.plot_config.set_logx(logx.into());
        self
    }
    fn set_logy<N: Into<f64>>(&mut self, logy: N) -> &mut Self {
        self.plot_config.set_logy(logy.into());
        self
    }
    fn set_labelx<S: Display>(&mut self, labelx: S) -> &mut Self {
        self.plot_config.set_labelx(labelx.to_string());
        self
    }
    fn set_labely<S: Display>(&mut self, labely: S) -> &mut Self {
        self.plot_config.set_labely(labely.to_string());
        self
    }
    fn set_rangex<S, T>(&mut self, left: S, right: T) -> &mut Self
    where
        f64: From<S>,
        f64: From<T>,
    {
        self.plot_config
            .set_rangex((f64::from(left), f64::from(right)));
        self
    }
    fn set_rangey<S, T>(&mut self, down: S, up: T) -> &mut Self
    where
        f64: From<S>,
        f64: From<T>,
    {
        self.plot_config
            .set_rangey((f64::from(down), f64::from(up)));
        self
    }
    fn set_style<S>(&mut self, style: S) -> &mut Self
    where
        S: TryInto<crate::configuration::plot::style::Style>,
        <S as TryInto<style::Style>>::Error: Debug,
    {
        let style: Style = style.try_into().unwrap();
        self.plot_config.set_style(style);
        self
    }
    fn set_dashtype(&mut self, dashtype: usize) -> &mut Self {
        self.plot_config.set_dashtype(dashtype);
        self
    }
    fn set_ticsx<T, S>(&mut self, ticsx: T) -> &mut Self
    where
        T: Into<Option<S>>,
        S: Display,
    {
        let ticsx: Option<S> = ticsx.into();
        self.plot_config.set_ticsx(ticsx.map(|t| t.to_string()));
        self
    }
    fn set_ticsy<T, S>(&mut self, ticsy: T) -> &mut Self
    where
        T: Into<Option<S>>,
        S: Display,
    {
        let ticsy: Option<S> = ticsy.into();
        self.plot_config.set_ticsy(ticsy.map(|t| t.to_string()));
        self
    }
    fn set_pause<T, S>(&mut self, pause: T) -> &mut Self
    where
        T: Into<Option<S>>,
        f64: From<S>,
    {
        let pause: Option<S> = pause.into();
        self.plot_config.set_pause(pause.map(f64::from));
        self
    }

    // Getting
    fn plot_extension(&self) -> Option<&OsStr> {
        self.plot_config.extension()
    }
    fn plot_path(&self) -> &Path {
        self.plot_config.path_buf()
    }
    fn title(&self) -> Option<&String> {
        self.plot_config.title().as_ref()
    }
    fn logx(&self) -> Option<f64> {
        *self.plot_config.logx()
    }
    fn logy(&self) -> Option<f64> {
        *self.plot_config.logy()
    }
    fn labelx(&self) -> Option<&String> {
        self.plot_config.labelx().as_ref()
    }
    fn labely(&self) -> Option<&String> {
        self.plot_config.labely().as_ref()
    }
    fn rangex(&self) -> Option<(f64, f64)> {
        *self.plot_config.rangex()
    }
    fn rangey(&self) -> Option<(f64, f64)> {
        *self.plot_config.rangey()
    }
    fn style(&self) -> &crate::configuration::plot::style::Style {
        self.plot_config.style()
    }
    fn dashtype(&self) -> Option<usize> {
        *self.plot_config.dashtype()
    }
    fn ticsx(&self) -> Option<&String> {
        self.plot_config.ticsx().as_ref()
    }
    fn ticsy(&self) -> Option<&String> {
        self.plot_config.ticsy().as_ref()
    }
    fn pause(&self) -> Option<f64> {
        *self.plot_config.pause()
    }

    ////////// SaveConfiguration /////////////////
    // Setting
    fn set_data_extension<S: AsRef<OsStr>>(&mut self, extension: S) -> &mut Self {
        self.save_config.set_extension(extension);
        self
    }
    fn set_header(&mut self, header: bool) -> &mut Self {
        self.save_config.set_header(header);
        self
    }
    fn set_date(&mut self, date: chrono::DateTime<chrono::Local>) -> &mut Self {
        self.save_config.set_date(date);
        self
    }
    fn set_id<S: Display>(&mut self, id: S) -> &mut Self {
        let id = id.to_string();
        self.plot_config.set_id(&id);
        self.save_config.set_id(id);
        self
    }

    // Getting
    fn data_extension(&self) -> Option<&OsStr> {
        self.save_config.extension()
    }
    fn data_path(&self) -> &Path {
        self.save_config.path_buf()
    }
    fn header(&self) -> bool {
        *self.save_config.header()
    }
    fn date(&self) -> &chrono::DateTime<chrono::Local> {
        self.save_config.date()
    }
    fn id(&self) -> Option<&String> {
        self.save_config.id().as_ref()
    }
    fn checked_id(&self) -> &String {
        self.save_config.checked_id()
    }

    ////////////////// CustomConfiguration ///////////////////
    fn set_custom<S: Display, T: Display>(&mut self, key: S, value: T) -> &mut Self {
        self.custom_config
            .insert(key.to_string(), value.to_string());
        self
    }

    fn custom<S: Display>(&self, key: S) -> Option<&String> {
        self.custom_config.get(&key.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn custom() {
        let mut config = Configuration::default();
        assert_eq!(config.custom("new"), None);

        config.set_custom("new", "new_option");
        assert_eq!(config.custom("new"), Some(&String::from("new_option")));
    }

    #[test]
    fn dashtype() {
        let mut config = Configuration::default();
        assert_eq!(config.dashtype(), None);

        config.set_dashtype(2);
        assert_eq!(config.dashtype(), Some(2));
    }

    #[test]
    fn extensions() {
        let mut config = Configuration::default();
        assert_eq!(config.data_extension().unwrap().to_str(), Some("txt"));
        assert_eq!(config.plot_extension().unwrap().to_str(), Some("gnu"));

        config.set_data_extension("my");
        assert_eq!(config.data_extension().unwrap().to_str(), Some("my"));
        config.set_plot_extension("my2");
        assert_eq!(config.plot_extension().unwrap().to_str(), Some("my2"));
    }

    #[test]
    fn date() {
        let mut config = Configuration::default();
        use chrono::{DateTime, Local};
        let _date: &DateTime<Local> = config.date();
        config.set_date(Local::now());
    }

    #[test]
    fn header() {
        let mut config = Configuration::default();
        assert_eq!(config.header(), true);

        config.set_header(false);
        assert_eq!(config.header(), false);
    }

    #[test]
    fn id() {
        let mut config = Configuration::default();

        config.set_id(1);
        assert_eq!(config.id(), Some(&1.to_string()));

        config.set_id(1.to_string());
        assert_eq!(config.id(), Some(&1.to_string()));

        config.set_id("1".to_string());
        assert_eq!(config.id(), Some(&String::from("1")));
    }

    #[test]
    fn labels() {
        let mut config = Configuration::default();
        assert_eq!(config.labelx(), None);

        let label = String::from("try");
        config.set_labelx(label.clone());
        assert_eq!(config.labelx(), Some(&label));
        let label = String::from("try2");
        config.set_xlabel(label.clone());
        assert_eq!(config.xlabel(), Some(&label));
        let label = String::from("try3");
        config.set_labely(label.clone());
        assert_eq!(config.labely(), Some(&label));
        let label = String::from("try4");
        config.set_ylabel(label.clone());
        assert_eq!(config.ylabel(), Some(&label));
    }

    #[test]
    fn log_axis() {
        let mut config = Configuration::default();
        assert_eq!(config.logx(), None);
        assert_eq!(config.logy(), None);
        assert_eq!(config.xlog(), None);
        assert_eq!(config.ylog(), None);

        config.set_logx(10.);
        assert_eq!(config.logx(), Some(10.));
        config.set_logy(9);
        assert_eq!(config.logy(), Some(9.));
        config.set_xlog(8);
        assert_eq!(config.xlog(), Some(8.));
        config.set_ylog(7);
        assert_eq!(config.ylog(), Some(7.));
    }

    #[test]
    fn pause() {
        let mut config = Configuration::default();
        assert_eq!(config.pause(), Some(-1.0));

        config.set_pause(2);
        assert_eq!(config.pause(), Some(2.0));
    }

    #[test]
    fn paths() {
        let mut config = Configuration::default();

        assert_eq!(config.id(), None);
        assert_eq!(config.data_extension().unwrap().to_str(), Some("txt"));

        assert_eq!(
            config.data_path().file_name().unwrap().to_str(),
            Some("none.txt")
        );
        assert_eq!(
            config.data_path().file_stem().unwrap().to_str(),
            Some("none")
        );
        assert_eq!(
            config.data_path().extension().unwrap().to_str(),
            Some("txt")
        );

        assert_eq!(
            config.plot_path().file_name().unwrap().to_str(),
            Some("none.gnu")
        );
        assert_eq!(
            config.data_path().file_stem().unwrap().to_str(),
            Some("none")
        );
        assert_eq!(
            config.plot_path().extension().unwrap().to_str(),
            Some("gnu")
        );

        config.set_id("testing".to_string());

        assert_eq!(
            config.plot_path().file_name().unwrap().to_str(),
            Some("testing.gnu")
        );
        assert_eq!(
            config.data_path().file_name().unwrap().to_str(),
            Some("testing.txt")
        );
    }

    #[test]
    fn ranges() {
        let mut config = Configuration::default();
        assert_eq!(config.rangex(), None);
        assert_eq!(config.rangey(), None);
        assert_eq!(config.xrange(), None);
        assert_eq!(config.yrange(), None);

        config.set_rangex(1, 2);
        assert_eq!(config.rangex(), Some((1., 2.)));
        config.set_rangey(3, 4.5);
        assert_eq!(config.rangey(), Some((3., 4.5)));
        config.set_xrange(3, -1.0);
        assert_eq!(config.xrange(), Some((3.0, -1.0)));
        config.set_yrange(4, 3);
        assert_eq!(config.yrange(), Some((4.0, 3.0)));
    }

    // #[test]
    // fn style() {
    //     use crate::configuration::plot::style::Style;
    //     let mut config = Configuration::default();
    //     assert_eq!(config.style(), &Style::Default);

    //     config.set_style("linespoints");
    //     assert_eq!(config.style(), &Style::Linespoints);
    //     config.set_style(9);
    //     assert_eq!(config.style(), &Style::Boxes);
    // }

    #[test]
    fn tics() {
        let mut config = Configuration::default();
        assert_eq!(config.ticsx(), Some(&"".to_string()));
        assert_eq!(config.ticsy(), Some(&"".to_string()));
        assert_eq!(config.xtics(), Some(&"".to_string()));
        assert_eq!(config.ytics(), Some(&"".to_string()));

        config.set_ticsx("try1");
        assert_eq!(config.ticsx(), Some(&"try1".to_string()));
        config.set_ticsy(9);
        assert_eq!(config.ticsy(), Some(&"9".to_string()));
        config.set_xtics(4);
        assert_eq!(config.xtics(), Some(&"4".to_string()));
        config.set_ytics("try2");
        assert_eq!(config.ytics(), Some(&"try2".to_string()));
    }

    #[test]
    fn title() {
        let mut config = Configuration::default();
        assert_eq!(config.title(), None);

        config.set_title("try");
        assert_eq!(config.title(), Some(&"try".to_string()));
        config.set_title(9);
        assert_eq!(config.title(), Some(&"9".to_string()));
    }
}

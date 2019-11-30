// Trait bounds
use crate::errors::SavingError;
use core::fmt::Display;

pub trait Preexplorable {
    // Needed methods

    fn save<S: Display>(&self, serie: S) -> Result<&Self, SavingError>;

    fn plot<S: Display>(&self, serie: S) -> Result<&Self, SavingError>;

    fn write_plot_script<S: Display>(&self, serie: S) -> Result<&Self, SavingError>;

    fn configuration(&mut self) -> &mut crate::configuration::Configuration;

    fn configuration_as_ref(&self) -> &crate::configuration::Configuration;


    // Implemented methods

    fn title<S: Display>(&mut self, title: S) -> &mut Self {
        self.configuration().title(title.to_string());
        self
    }
    fn logx<N: Into<f64>>(&mut self, logx: N) -> &mut Self {
        self.configuration().logx(logx.into());
        self
    }
    fn logy<N: Into<f64>>(&mut self, logy: N) -> &mut Self {
        self.configuration().logy(logy.into());
        self
    }
    fn labelx<S: Display>(&mut self, labelx: S) -> &mut Self {
        self.configuration().labelx(labelx.to_string());
        self
    }
    fn labely<S: Display>(&mut self, labely: S) -> &mut Self {
        self.configuration().labely(labely.to_string());
        self
    }
    fn extension<S: Display>(&mut self, extension: S) -> &mut Self {
        self.configuration().extension(extension.to_string());
        self
    }
    fn header(&mut self, header: bool) -> &mut Self {
        self.configuration().header(header);
        self
    }
    fn style<S>(&mut self, style: S) -> &mut Self 
    where
        crate::configuration::plot::style::Style: From<S>,
    {
        self.configuration().style(crate::configuration::plot::style::Style::from(style));
        self
    }
    fn dashtype(&mut self, dashtype: usize) -> &mut Self {
        self.configuration().dashtype(dashtype);
        self
    }

    fn base_plot_script(&self) -> String {
        self.configuration_as_ref().base_plot_script()
    }

    fn get_title(&self) -> Option<&String> {
        self.configuration_as_ref().get_title()
    }
    fn get_logx(&self) -> Option<f64> {
        self.configuration_as_ref().get_logx()
    }
    fn get_logy(&self) -> Option<f64> {
        self.configuration_as_ref().get_logy()
    }
    fn get_labelx(&self) -> Option<&String> {
        self.configuration_as_ref().get_labelx()
    }
    fn get_labely(&self) -> Option<&String> {
        self.configuration_as_ref().get_labely()
    }
    fn get_extension(&self) -> &str {
        self.configuration_as_ref().get_extension()
    }
    fn get_header(&self) -> bool {
        self.configuration_as_ref().get_header()
    }
    fn get_style(&self) -> &crate::configuration::plot::style::Style {
        self.configuration_as_ref().get_style()
    }
    fn get_dashtype(&self) -> Option<usize> {
        self.configuration_as_ref().get_dashtype()
    }
}

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

    fn set_title<S: Display>(&mut self, title: S) -> &mut Self {
        self.configuration().set_title(title.to_string());
        self
    }
    fn set_logx<N: Into<f64>>(&mut self, logx: N) -> &mut Self {
        self.configuration().set_logx(logx.into());
        self
    }
    fn set_logy<N: Into<f64>>(&mut self, logy: N) -> &mut Self {
        self.configuration().set_logy(logy.into());
        self
    }
    fn set_labelx<S: Display>(&mut self, labelx: S) -> &mut Self {
        self.configuration().set_labelx(labelx.to_string());
        self
    }
    fn set_labely<S: Display>(&mut self, labely: S) -> &mut Self {
        self.configuration().set_labely(labely.to_string());
        self
    }

    fn set_extension<S: Display>(&mut self, extension: S) -> &mut Self {
        self.configuration().set_extension(extension.to_string());
        self
    }
    fn set_header(&mut self, header: bool) -> &mut Self {
        self.configuration().set_header(header);
        self
    }


    fn title(&self) -> Option<String> {
        self.configuration_as_ref().title()
    }
    fn logx(&self) -> Option<f64> {
        self.configuration_as_ref().logx()
    }
    fn logy(&self) -> Option<f64> {
        self.configuration_as_ref().logy()
    }
    fn labelx(&self) -> Option<&str> {
        self.configuration_as_ref().labelx()
    }
    fn labely(&self) -> Option<&str> {
        self.configuration_as_ref().labely()
    }
    fn extension(&self) -> &str {
        self.configuration_as_ref().extension()
    }
    fn header(&self) -> bool {
        self.configuration_as_ref().header()
    }
}

// Trait bounds
use crate::errors::SavingError;
use core::fmt::Display;

pub trait Preexplorable {
    fn save<S: Display>(&self, serie: &S) -> Result<&Self, SavingError>;

    fn plot<S: Display>(&self, serie: &S) -> Result<&Self, SavingError>;

    fn write_plot_script<S: Display>(&self, serie: &S) -> Result<&Self, SavingError>;

    fn configuration(&mut self) -> &mut crate::configuration::Configuration;

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
}

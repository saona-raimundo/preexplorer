// Trait bounds
use crate::errors::SavingError;
use core::fmt::Display;

pub trait PlotableStructure {
    fn save<S: Display>(self, serie: &S) -> Result<(), SavingError>;

    fn plot<S: Display>(self, serie: &S) -> Result<(), SavingError>;

    fn write_plot_script<S: Display>(&self, serie: &S) -> Result<(), SavingError>;
}

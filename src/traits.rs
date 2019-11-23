// Trait bounds
use crate::errors::SavingError;
use core::fmt::Display;

pub trait Preexplorable {
    fn save<S: Display>(&self, serie: &S) -> Result<&Self, SavingError>;

    fn plot<S: Display>(&self, serie: &S) -> Result<&Self, SavingError>;

    fn write_plot_script<S: Display>(&self, serie: &S) -> Result<&Self, SavingError>;
}

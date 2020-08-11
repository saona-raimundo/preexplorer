//! Most basic explorable structure: a sequence of values.
//!
//! # Remarks
//!
//! With the ``prelude`` module, we can easily convert ``IntoIterator``s
//! into ``Sequence`` for ease of use. The same can be achieved with the
//! ``new`` method.
//!
//! # Examples
//!
//! Quick plot.
//! ```no_run
//! use preexplorer::prelude::*;
//! (0..10).preexplore().plot("my_identifier").unwrap();
//! ```
//!
//! Compare ``Sequence``s.
//! ```no_run
//! use preexplorer::prelude::*;
//! pre::Sequences::new(vec![
//!     (0..10).preexplore(),
//!     (0..10).preexplore(),
//!     ])
//!     .plot("my_identifier").unwrap();
//! ```

// Traits
use core::ops::Add;
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;

/// Compare various ``Sequence``s.
pub mod comparison;
/// Sequence of values with an associated error.
pub mod error;
/// Sequence of violin plots.
pub mod violin;
/// Sequence of histograms.
pub mod bin;

pub use comparison::Sequences;
pub use error::{SequenceError, SequenceErrors};
pub use violin::{SequenceViolin};
pub use bin::{SequenceBin};

/// Sequence of values.
#[derive(Debug, PartialEq, Clone)]
pub struct Sequence<T>
where
    T: Display + Clone,
{
    data: Vec<T>,
    config: crate::configuration::Configuration,
}

impl<T> Sequence<T>
where
    T: Display + Clone,
{
    /// Create a new ``Sequence``.
    ///
    /// # Examples
    ///
    /// From a complicated computation.
    /// ```
    /// use preexplorer::prelude::*;
    /// let data = (0..10).map(|i| i * i + 1);
    /// let seq = pre::Sequence::new(data);
    /// ```
    pub fn new<I>(data: I) -> Sequence<T>
    where
        I: IntoIterator<Item = T>,
    {
        let data: Vec<T> = data.into_iter().collect();
        let config = crate::configuration::Configuration::default();

        Sequence { data, config }
    }
}

impl<T> Add for Sequence<T>  
where
    T: Display + Clone,
{
    type Output = crate::Sequences<T>;

    fn add(self, other: crate::Sequence<T>) -> crate::Sequences<T> { 
        let mut cmp = self.into();
        cmp += other;
        cmp
    }
}

impl<T> Configurable for Sequence<T>
where
    T: Display + Clone,
{
    fn configuration_mut(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

impl<T> Saveable for Sequence<T>
where
    T: Display + Clone,
{
    fn plotable_data(&self) -> String {
        let mut plotable_data = String::new();

        for (counter, value) in self.data.clone().into_iter().enumerate() {
            plotable_data.push_str(&format!("{}\t{}\n", counter, value));
        }

        plotable_data
    }
}

impl<T> Plotable for Sequence<T>
where
    T: Display + Clone,
{
    fn plot_script(&self) -> String {
        let mut gnuplot_script = self.opening_plot_script();

        let dashtype = match self.dashtype() {
            Some(dashtype) => dashtype,
            None => 1,
        };
        gnuplot_script += &format!(
            "plot {:?} with {} dashtype {} \n",
            self.data_path(),
            self.style(),
            dashtype,
        );
        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_style() {
        let data = 0..2;
        let mut seq = Sequence::new(data);
        seq.set_style("points");

        assert_eq!(
            &crate::configuration::plot::style::Style::Points,
            seq.style()
        );
    }
}

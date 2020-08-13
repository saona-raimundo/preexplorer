//! A sequence of values with a given error.
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
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::ops::Add;

// Structs
use average::Variance;

/// Compare various ``SequenceError``s.
pub mod comparison;

pub use comparison::SequenceErrors;

/// Sequence of values with a given error.
#[derive(Debug, PartialEq, Clone)]
pub struct SequenceError {
    data: Vec<(f64, f64)>,
    config: crate::configuration::Configuration,
}

impl SequenceError {
    /// Create a new ``SequenceError``.
    ///
    /// # Examples
    ///
    /// From a complicated computation.
    /// ```
    /// use preexplorer::prelude::*;
    /// let data = (0..10).map(|i| i * i + 1);
    /// let seq = pre::SequenceError::new(data);
    /// ```
    pub fn new<I, J>(data: I) -> SequenceError
    where
        I: IntoIterator<Item = J>,
        J: IntoIterator<Item = f64>,
    {
        let data: Vec<(f64, f64)> = data
            .into_iter()
            .map(|j| {
                let v: Variance = j.into_iter().collect();
                (v.mean(), v.error())
            })
            .collect();
        let config = crate::configuration::Configuration::default();

        SequenceError { data, config }
    }
}

impl Add for SequenceError {
    type Output = crate::SequenceErrors;

    fn add(self, other: crate::SequenceError) -> crate::SequenceErrors {
        let mut cmp = self.into();
        cmp += other;
        cmp
    }
}

impl Configurable for SequenceError {
    fn configuration_mut(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

impl Saveable for SequenceError {
    fn plotable_data(&self) -> String {
        let mut plotable_data = String::new();

        for (counter, (value, error)) in self.data.clone().into_iter().enumerate() {
            plotable_data.push_str(&format!("{}\t{}\t{}\n", counter, value, error));
        }

        plotable_data
    }
}

impl Plotable for SequenceError {
    fn plot_script(&self) -> String {
        let mut gnuplot_script = self.opening_plot_script();

        let dashtype = match self.dashtype() {
            Some(dashtype) => dashtype,
            None => 1,
        };
        gnuplot_script += &format!(
            "plot {:?} using 1:2 with {} dashtype {}, \"\" using 1:2:3 with yerrorbars \n",
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
        let data = vec![vec![0., 1.], vec![0., 1., 2.], vec![3., 4., 5.]];
        let mut seq = SequenceError::new(data);
        seq.set_style("points");

        assert_eq!(
            &crate::configuration::plot::style::Style::Points,
            seq.style()
        );
    }
}

//! Indexed collection of values.
//!
//! # Remarks
//!
//! With the ``prelude`` module, we can easily convert a tuple of ``IntoIterator``s
//! into ``Process`` for ease of use. The same can be achieved with the
//! ``new`` method.
//!
//! # Examples
//!
//! Quick plot.
//! ```no_run
//! use preexplorer::prelude::*;
//! ((0..10), (0..10)).preexplore().plot("my_identifier").unwrap();
//! ```
//!
//! Compare ``Process``es.
//! ```no_run
//! use preexplorer::prelude::*;
//! pre::Processes::new(vec![
//!     ((0..10), (0..10)).preexplore(),
//!     ((0..10), (0..10)).preexplore(),
//!     ])
//!     .plot("my_identifier").unwrap();
//! ```

// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;
use core::ops::Add;

/// Compare various ``Process``es.
pub mod comparison;
/// Process of values with an associated error.
pub mod error;
/// Process of histograms.
pub mod bin;

pub use comparison::Processes;
pub use error::{ProcessError, ProcessErrors};
pub use bin::{ProcessBin};

/// Indexed sequence of values.
#[derive(Debug, PartialEq, Clone)]
pub struct Process<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    domain: Vec<T>,
    image: Vec<S>,
    config: crate::configuration::Configuration,
}

impl<T, S> Process<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    /// Create a new ``Process``.
    ///
    /// # Examples
    ///
    /// From a complicated computation.
    /// ```
    /// use preexplorer::prelude::*;
    /// let data = (0..10).map(|i| i * i + 1);
    /// let seq = pre::Process::new((0..10), data);
    /// ```
    pub fn new<I, J>(domain: I, image: J) -> Process<T, S>
    where
        I: IntoIterator<Item = T>,
        J: IntoIterator<Item = S>,
    {
        let domain: Vec<T> = domain.into_iter().collect();
        let image: Vec<S> = image.into_iter().collect();
        let config = crate::configuration::Configuration::default();

        Process {
            domain,
            image,
            config,
        }
    }
}

impl<T, S> Add for Process<T, S>  
where
    T: Display + Clone,
    S: Display + Clone,
{
    type Output = crate::Processes<T, S>;

    fn add(self, other: crate::Process<T, S>) -> crate::Processes<T, S> { 
        let mut cmp = self.into();
        cmp += other;
        cmp
    }
}

impl<T, S> Configurable for Process<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn configuration_mut(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

impl<T, S> Saveable for Process<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn plotable_data(&self) -> String {
        let mut plotable_data = String::new();
        for (time, value) in self.domain.clone().into_iter().zip(self.image.clone()) {
            plotable_data.push_str(&format!("{}\t{}\n", time, value));
        }
        plotable_data
    }
}

impl<T, S> Plotable for Process<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn plot_script(&self) -> String {
        let mut gnuplot_script = self.opening_plot_script();

        let dashtype = match self.dashtype() {
            Some(dashtype) => dashtype,
            None => 1,
        };

        gnuplot_script += &format!(
            "plot {:?} using 1:2 with {} dashtype {}\n",
            self.data_path(),
            self.style(),
            dashtype,
        );
        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

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

// Structs
use average::Variance;

/// Compare various ``Process``es.
pub mod comparison;

pub use comparison::ProcessErrors;

/// Indexed sequence of values.
#[derive(Debug, PartialEq, Clone)]
pub struct ProcessError<T>
where
    T: Display + Clone,
{
    domain: Vec<T>,
    image: Vec<(f64, f64)>,
    config: crate::configuration::Configuration,
}

impl<T> ProcessError<T>
where
    T: Display + Clone,
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
    pub fn new<I, J, K>(domain: I, image: J) -> ProcessError<T>
    where
        I: IntoIterator<Item = T>,
        J: IntoIterator<Item = K>,
        K: IntoIterator<Item = f64>,
    {
        let domain: Vec<T> = domain.into_iter().collect();
        let image: Vec<(f64, f64)> = image
            .into_iter()
            .map(|k| {
                let v: Variance = k.into_iter().collect();
                (v.mean(), v.error())
            })
            .collect();
        let config = crate::configuration::Configuration::default();

        ProcessError {
            domain,
            image,
            config,
        }
    }
}

impl<T> Add for ProcessError<T>
where
    T: Display + Clone,
{
    type Output = crate::ProcessErrors<T>;

    fn add(self, other: crate::ProcessError<T>) -> crate::ProcessErrors<T> {
        let mut cmp = self.into();
        cmp += other;
        cmp
    }
}

impl<T> Configurable for ProcessError<T>
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

impl<T> Saveable for ProcessError<T>
where
    T: Display + Clone,
{
    fn plotable_data(&self) -> String {
        let mut raw_data = String::new();
        for (time, (value, error)) in self.domain.clone().into_iter().zip(self.image.clone()) {
            raw_data.push_str(&format!("{}\t{}\t{}\n", time, value, error));
        }
        raw_data
    }
}

impl<T> Plotable for ProcessError<T>
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
            "plot {:?} using 1:2 with {} dashtype {}, \"\" using 1:($2+$3):($2-$3) with filledcurves fs transparent solid 0.5 linecolor rgb \"dark-grey\"\n",
            self.data_path(),
            self.style(),
            dashtype,
        );
        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

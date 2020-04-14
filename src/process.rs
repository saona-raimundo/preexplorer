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
pub use crate::traits::{Comparison, Configurable, Plotable, Saveable};
use core::fmt::Display;

/// Compare various ``Process``es.
pub mod comparison;

pub use comparison::Processes;

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

    /// Convert to ``Processes`` quickly.
    pub fn to_comparison(&self) -> crate::process::comparison::Processes<T, S> {
        self.clone().into()
    }

    /// Compare your ``Process`` with various ``Process``es.
    ///
    /// # Remarks
    ///
    /// Titles of ``Process``es involved in a ``Processes`` are presented as legends.
    ///
    /// # Examples
    ///
    /// Compare many ``Process``es by gathering all first (in some ``IntoIterator``).
    /// ```no_run
    /// use preexplorer::prelude::*;
    /// let first_pro = ((0..10), (0..10)).preexplore().title("legend").to_owned();
    /// let many_pros = (0..5).map(|_| ((0..10), (0..10)).preexplore());
    /// let mut processes = first_pro.compare_with(many_pros);
    /// processes.title("Main title");
    /// ```
    pub fn compare_with<K>(self, others: K) -> crate::process::comparison::Processes<T, S>
    where
        K: IntoIterator<Item = crate::process::Process<T, S>>,
    {
        let mut comp: Processes<T, S> = self.into();
        comp.add_many(others);
        comp
    }
}

impl<T, S> Configurable for Process<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn configuration(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration_as_ref(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

impl<T, S> Saveable for Process<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn plotable_data(&self) -> String {
        let mut raw_data = String::new();
        for (time, value) in self.domain.clone().into_iter().zip(self.image.clone()) {
            raw_data.push_str(&format!("{}\t{}\n", time, value));
        }
        raw_data
    }
}

impl<T, S> Plotable for Process<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn plot_script(&self) -> String {
        let mut gnuplot_script = self.opening_plot_script();

        let dashtype = match self.get_dashtype() {
            Some(dashtype) => dashtype,
            None => 1,
        };

        gnuplot_script += &format!(
            "plot {:?} using 1:2 with {} dashtype {}\n",
            self.get_data_path(),
            self.get_style(),
            dashtype,
        );
        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

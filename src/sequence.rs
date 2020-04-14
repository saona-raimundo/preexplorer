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
pub use crate::traits::{Comparison, Configurable, Plotable, Saveable};
use core::fmt::Display;

/// Compare various ``Sequence``s.
pub mod comparison;

pub use comparison::Sequences;

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

    /// Convert to ``Sequences`` quickly.
    pub fn to_comparison(&self) -> crate::sequence::comparison::Sequences<T> {
        self.clone().into()
    }

    /// Compare your ``Sequence`` with various ``Sequence``s.
    ///
    /// # Remarks
    ///
    /// Titles of ``Sequence``s involved in a ``Sequences`` are presented as legends.
    ///
    /// # Examples
    ///
    /// Compare many ``Sequence``s by gathering all first (in some ``IntoIterator``).
    /// ```no_run
    /// use preexplorer::prelude::*;
    /// let first_seq = (0..10).preexplore().title("legend").to_owned();
    /// let many_seqs = (0..5).map(|_| (0..10).preexplore());
    /// let mut sequences = first_seq.compare_with(many_seqs);
    /// sequences.title("Main title");
    /// ```
    pub fn compare_with<J>(self, others: J) -> crate::sequence::comparison::Sequences<T>
    where
        J: IntoIterator<Item = crate::sequence::Sequence<T>>,
    {
        let mut comp: Sequences<T> = self.into();
        comp.add_many(others);
        comp
    }
}

impl<T> Configurable for Sequence<T>
where
    T: Display + Clone,
{
    fn configuration(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration_as_ref(&self) -> &crate::configuration::Configuration {
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

        let dashtype = match self.get_dashtype() {
            Some(dashtype) => dashtype,
            None => 1,
        };
        gnuplot_script += &format!(
            "plot {:?} with {} dashtype {} \n",
            self.get_data_path(),
            self.get_style(),
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
        seq.style("points");

        assert_eq!(
            &crate::configuration::plot::style::Style::Points,
            seq.get_style()
        );
    }
}

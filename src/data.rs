//! Generic multi-dimensional data.
//!
//! # Remarks
//!
//! It should be used with the ``plot_later`` command, writting the perfect
//! plot script by interacting with gnuplot directly.
//!
//! # Examples
//!
//! Save data and plot script for posterior analysis.
//! ```no_run
//! # use preexplorer::prelude::*;
//! # use ndarray::array;
//! let data = array![
//!     [1, 2, 3, 4, 5],
//!     [2, 5, 6, 7, 8],
//!     [3, 11, 12, 13, 14],
//! ];
//! let dim = 5;
//!
//! pre::Data::new(data.iter(), dim)
//!     .plot_later("my_identifier")
//!     .unwrap();
//! ```

// Structs
use crate::errors::SavingError;

// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;

/// Generic multi-dimensional data.
///
#[derive(Debug, PartialEq, Clone)]
pub struct Data<T>
where
    T: Display,
{
    data: Vec<T>,
    config: crate::configuration::Configuration,
    dim: usize,
}

impl<T> Data<T>
where
    T: Display,
{
    /// Create a new ``Data``.
    ///
    /// The parameter ``dim`` represents the dimension of the data.
    /// Consecutive values are read as part of the same data point,
    /// meaning that a set of data {x = (x_1, x_2, ..., x_d)}
    /// should be given in an ``IntoIterator`` that gives the values in
    /// the following order: x_1, x_2, ..., x_d, y_1, y_2, ..., y_d, etc.
    pub fn new<I>(data: I, dim: usize) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let data: Vec<T> = data.into_iter().collect();
        let config = crate::configuration::Configuration::default();
        Data { data, config, dim }
    }
}

impl<T> Configurable for Data<T>
where
    T: Display,
{
    fn configuration(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }

    fn configuration_as_ref(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

impl<T> Saveable for Data<T>
where
    T: Display + Clone,
{
    fn plotable_data(&self) -> String {
        let mut raw_data = String::new();

        let mut counter = 0;
        for value in self.data.clone() {
            raw_data.push_str(&format!("{}\t", value));
            counter += 1;
            if counter == self.dim {
                counter = 0;
                raw_data.push_str("\n");
            }
        }

        raw_data
    }
}

impl<T> Plotable for Data<T>
where
    T: Display + Clone,
{
    /// Call ``plot_later`` and retunrs error, since generic data
    /// should be plotted by hand interacting with gnuplot.
    fn plot<S: Display>(&mut self, id: S) -> Result<&mut Self, SavingError> {
        self.plot_later(id)?;

        let message = format!("Tried to plot general data: do it directly with gnuplot. A preliminar gnuplot script is located in {:?}", self.get_plot_path());
        Err(std::io::Error::new(std::io::ErrorKind::Other, message).into())
    }

    /// Basic plot script with the instructions to search for the perfect
    /// plot in internet.
    fn plot_script(&self) -> String {
        let mut gnuplot_script = self.opening_plot_script();

        gnuplot_script +=
            "\n# Visit http://www.gnuplotting.org and search for the correct plotting command!\n";
        gnuplot_script += "\n# To get the plot, run the following command:";
        gnuplot_script += &format!("\n# gnuplot {:?} \n\n", self.get_plot_path());
        gnuplot_script += &format!("plot {:?} \n", self.get_data_path());

        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

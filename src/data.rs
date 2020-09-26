// Structs
use crate::errors::PreexplorerError;

// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;

/// Generic multi-dimensional data.
///
/// # Remarks
///
/// It should be used with the [plot_later] command, writting the perfect
/// plot script by interacting with gnuplot directly.
///
/// # Examples
///
/// Save data and plot script for posterior analysis.
/// ```no_run
/// # use preexplorer::prelude::*;
/// # use ndarray::array;
/// let data = array![
///     [1, 2, 3, 4, 5],
///     [2, 5, 6, 7, 8],
///     [3, 11, 12, 13, 14],
/// ];
/// let dim = 5;
///
/// pre::Data::new(data.iter(), dim)
///     .plot_later("my_identifier")
///     .unwrap();
/// ```
///
/// [plot_later]: trait.Plotable.html#method.plot_later
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
    /// Constructs a new ``Data<T>``.
    ///
    /// The parameter ``dim`` represents the dimension of the data.
    /// Consecutive values are read as part of the same data point,
    /// meaning that a set of data {x = (x_1, x_2, ..., x_d)}
    /// should be given in an [IntoIterator] that gives the values in
    /// the following order: x_1, x_2, ..., x_d, y_1, y_2, ..., y_d, etc.
    ///
    /// [IntoIterator]: https://doc.rust-lang.org/core/iter/trait.IntoIterator.html
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
    fn configuration_mut(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }

    fn configuration(&self) -> &crate::configuration::Configuration {
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
    /// Calls [plot_later] method and retunrs error since generic data
    /// should be plotted by hand interacting with gnuplot.
    ///
    /// [plot_later]: trait.Plotable.html#method.plot_later
    fn plot<S: Display>(&mut self, id: S) -> Result<&mut Self, PreexplorerError> {
        self.plot_later(id)?;

        let message = format!("Tried to plot general data: do it directly with gnuplot. A preliminar gnuplot script is located in {:?}", self.plot_path());
        Err(PreexplorerError::Plotting(
            std::io::Error::new(std::io::ErrorKind::Other, message),
        ))
    }

    fn plot_script(&self) -> String {
        let mut gnuplot_script = self.opening_plot_script();

        gnuplot_script +=
            "\n# Visit http://www.gnuplotting.org and search for the correct plotting command!\n";
        gnuplot_script += "\n# To get the plot, run the following command:";
        gnuplot_script += &format!("\n# gnuplot {:?} \n\n", self.plot_path());
        gnuplot_script += &format!("plot {:?} \n", self.data_path());

        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

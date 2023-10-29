// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;

/// 3-dimensional surface by plotting constant z slices, called contours, on a 2-dimensional format.
///
/// Check out [`Contour` documentation] of gnuplot for more options.
///
///
/// # Examples
///
/// Quick plot.
/// ```no_run
/// use itertools::iproduct;
/// use preexplorer::prelude::*;
/// let values = iproduct!(0..10, 0..5).map(|(x, y)| x + y);
/// pre::Contour::new(0..10, 0..5, values).plot("my_identifier").unwrap();
/// ```
///
/// [`Contour` documentation]: http://gnuplot.info/docs_5.5/loc10902.html
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Contour<T, S, U>
where
    T: Display + Clone,
    S: Display + Clone,
    U: Display + Clone,
{
    xs: Vec<T>,
    ys: Vec<S>,
    values: Vec<U>,
    config: crate::configuration::Configuration,
}

impl<T, S, U> Contour<T, S, U>
where
    T: Display + Clone,
    S: Display + Clone,
    U: Display + Clone,
{
    /// Constructs a new ``Contour<T, S, U>``.
    ///
    /// # Panics
    ///
    /// The number of values must be equal to the dimension of the grid
    /// given by the cartesian product of ``xs`` and ``ys``.
    ///
    /// # Examples
    ///
    /// From a complicated computation.
    /// ```
    /// # use itertools::iproduct;
    /// use preexplorer::prelude::*;
    /// let values = iproduct!(0..10, 0..5).map(|(x, y)| x + y);
    /// let heatmap = pre::Contour::new(0..10, 0..5, values);
    /// ```
    pub fn new<I, J, K>(xs: I, ys: J, values: K) -> Contour<T, S, U>
    where
        I: IntoIterator<Item = T>,
        J: IntoIterator<Item = S>,
        K: IntoIterator<Item = U>,
    {
        let xs: Vec<T> = xs.into_iter().collect();
        let ys: Vec<S> = ys.into_iter().collect();
        let values: Vec<U> = values.into_iter().collect();

        debug_assert!(
            xs.len() * ys.len() == values.len(),
            "The numbers of values ({}) does not match the grid ({}x{})",
            values.len(),
            xs.len(),
            ys.len()
        );

        let mut config = crate::configuration::Configuration::default();
        config.set_style("lines").unwrap();

        Contour {
            xs,
            ys,
            values,
            config,
        }
    }
}


impl<T, S, U> Configurable for Contour<T, S, U>
where
    T: Display + Clone,
    S: Display + Clone,
    U: Display + Clone,
{
    fn configuration_mut(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

impl<T, S, U> Saveable for Contour<T, S, U>
where
    T: Display + Clone,
    S: Display + Clone,
    U: Display + Clone,
{
    fn plotable_data(&self) -> String {
        // Initial warning
        if self.xs.is_empty() {
            eprintln!("Warning: There is no data.");
        }

        let mut plotable_data = String::new();
        for i in 0..self.xs.len() {
            for j in 0..self.ys.len() {
                plotable_data.push_str(&format!(
                    "{}\t{}\t{}\n",
                    self.xs[i],
                    self.ys[j],
                    self.values[i * self.ys.len() + j]
                ));
            }
            plotable_data.push_str("\n");
        }
        plotable_data
    }
}

impl<T, S, U> Plotable for Contour<T, S, U>
where
    T: Display + Clone,
    S: Display + Clone,
    U: Display + Clone,
{
    fn plot_script(&self) -> String {
        let mut gnuplot_script = self.opening_plot_script();

        gnuplot_script += "set surface # unset to plot only isolines\n";
        gnuplot_script += "set contour\n";
        gnuplot_script += &format!("splot {:?} using 1:2:3 with {}\n", self.data_path(), self.style());
        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

impl<T> From<ndarray::Array2<T>> for Contour<usize, usize, T>
where
    T: Display + Clone,
{
    fn from(array: ndarray::Array2<T>) -> Self {
        let shape = array.shape();

        let xs: Vec<usize> = (0..shape[0]).collect();
        let ys: Vec<usize> = (0..shape[1]).rev().collect();
        let values: Vec<T> = array.t().iter().cloned().collect();

        Contour::new(xs, ys, values)
    }
}

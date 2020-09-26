// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;
use core::ops::Add;

pub mod comparison;

pub use comparison::Heatmaps;

/// Indexed sequence of values.
///
/// # Examples
///
/// Quick plot.
/// ```no_run
/// # use itertools::iproduct;
/// use preexplorer::prelude::*;
/// let values = iproduct!(0..10, 0..5).map(|(x, y)| x + y);
/// pre::Heatmap::new(0..10, 0..5, values).plot("my_identifier").unwrap();
/// ```
///
/// Compare [Heatmap] structs.
/// ```no_run
/// # use itertools::iproduct;
/// use preexplorer::prelude::*;
/// pre::Heatmaps::new(vec![
///     pre::Heatmap::new(0..10, 0..5, iproduct!(0..10, 0..5).map(|(x, y)| x + y)),
///     pre::Heatmap::new(0..10, 0..5, iproduct!(0..10, 0..5).map(|(x, y)| x * y)),
///     ])
///     .plot("my_identifier").unwrap();
/// ```
///
/// [Heatmap]: struct.Heatmap.html
#[derive(Debug, PartialEq, Clone)]
pub struct Heatmap<T, S, U>
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

impl<T, S, U> Heatmap<T, S, U>
where
    T: Display + Clone,
    S: Display + Clone,
    U: Display + Clone,
{
    /// Constructs a new ``Heatmap<T, S, U>``.
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
    /// let heatmap = pre::Heatmap::new(0..10, 0..5, values);
    /// ```
    pub fn new<I, J, K>(xs: I, ys: J, values: K) -> Heatmap<T, S, U>
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

        let config = crate::configuration::Configuration::default();

        Heatmap {
            xs,
            ys,
            values,
            config,
        }
    }
}

impl<T, S, U> Add for Heatmap<T, S, U>
where
    T: Display + Clone,
    S: Display + Clone,
    U: Display + Clone,
{
    type Output = crate::Heatmaps<T, S, U>;

    fn add(self, other: crate::Heatmap<T, S, U>) -> crate::Heatmaps<T, S, U> {
        let mut cmp = self.into();
        cmp += other;
        cmp
    }
}

impl<T, S, U> Configurable for Heatmap<T, S, U>
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

impl<T, S, U> Saveable for Heatmap<T, S, U>
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
        }
        plotable_data
    }
}

impl<T, S, U> Plotable for Heatmap<T, S, U>
where
    T: Display + Clone,
    S: Display + Clone,
    U: Display + Clone,
{
    fn plot_script(&self) -> String {
        let mut gnuplot_script = self.opening_plot_script();

        gnuplot_script += &format!("plot {:?} using 1:2:3 with image\n", self.data_path(),);
        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

impl<T> From<ndarray::Array2<T>> for Heatmap<usize, usize, T>
where
    T: Display + Clone,
{
    fn from(array: ndarray::Array2<T>) -> Self {
        let shape = array.shape();

        let xs: Vec<usize> = (0..shape[0]).collect();
        let ys: Vec<usize> = (0..shape[1]).rev().collect();
        let values: Vec<T> = array.t().iter().cloned().collect();

        Heatmap::new(xs, ys, values)
    }
}

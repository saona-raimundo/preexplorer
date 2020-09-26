// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::ops::Add;

// Structs
use average::Variance;

/// Compare various ``SequenceError``s.
pub mod comparison;

pub use comparison::SequenceErrors;

/// Sequence of values with a given error.
///
/// # Examples
///
/// Quick plot.
/// ```no_run
/// use preexplorer::prelude::*;
/// let data = (0..10).map(|i| (i..10 + i));
/// let seq_err = pre::SequenceError::new(data).plot("my_identifier").unwrap();
/// ```
///
/// Compare [SequenceError] structs.
/// ```no_run
/// use preexplorer::prelude::*;
/// pre::SequenceErrors::new(vec![
///     pre::SequenceError::new((0..10).map(|i| (i..10 + i))),
///     pre::SequenceError::new((0..10).map(|i| (i..10 + i))),
///     ])
///     .plot("my_identifier").unwrap();
/// ```
///
/// [SequenceError]: struct.SequenceError.html
#[derive(Debug, PartialEq, Clone)]
pub struct SequenceError {
    data: Vec<(f64, f64)>,
    config: crate::configuration::Configuration,
}

impl SequenceError {
    /// Constructs a new ``SequenceError`` from data.
    ///
    /// Each dataset is processed so that the final plot shows the mean of the data set and
    /// an error bar of one standard deviation.
    ///
    /// # Examples
    ///
    /// From a complicated computation.
    /// ```
    /// use preexplorer::prelude::*;
    /// let data = (0..10).map(|i| i..10 + i);
    /// let seq_err = pre::SequenceError::new(data);
    /// ```
    pub fn new<I, J, T>(data: I) -> SequenceError
    where
        I: IntoIterator<Item = J>,
        J: IntoIterator<Item = T>,
        T: Into<f64>,
    {
        let data: Vec<(f64, f64)> = data
            .into_iter()
            .map(|j| {
                let v: Variance = j.into_iter().map(|t| t.into()).collect();
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
        // Initial warning
        if self.data.is_empty() {
            eprintln!("Warning: There is no data.");
        }

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

impl<T> From<crate::Densities<T>> for SequenceError
where
    T: Into<f64> + core::fmt::Display + Clone,
{
    fn from(mut densities: crate::Densities<T>) -> Self {
        let data: Vec<Vec<f64>> = (0..densities.data_set.len())
            .map(|i| {
                densities.data_set[i]
                    .realizations
                    .iter()
                    .map(|t| t.clone().into())
                    .collect()
            })
            .collect();
        let mut seq_err = SequenceError::new(data);
        let config = seq_err.configuration_mut();
        *config = densities.configuration_mut().clone();
        seq_err
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

    #[test]
    fn from_densitites() {
        use crate::prelude::*;
        let many_dens = (0..5).map(|_| pre::Density::new(0..10));
        let mut densities: pre::Densities<u32> = pre::Densities::new(many_dens);
        densities.set_title("My title");
        let seq_err = pre::SequenceError::from(densities.clone());

        assert_eq!(seq_err.title(), densities.title());
    }
}

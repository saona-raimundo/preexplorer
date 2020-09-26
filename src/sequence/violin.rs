// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;
use core::ops::Add;

/// Compare various ``Sequence``s.
pub mod comparison;

pub use comparison::SequenceViolins;

/// Sequence of violin plots.
///
/// # Examples
///
/// Quick plot.
/// ```no_run
/// use preexplorer::prelude::*;
/// let data = (0..10).map(|i| (i..10 + i));
/// pre::SequenceViolin::new(data).plot("my_identifier").unwrap();
/// ```
///
/// Compare ``SequenceViolin``s.
/// ```no_run
/// use preexplorer::prelude::*;
/// pre::SequenceViolins::new(vec![
///     pre::SequenceViolin::new((0..10).map(|i| (i..10 + i))),
///     pre::SequenceViolin::new((0..10).map(|i| (i..10 + 2 * i))),
///     ])
///     .plot("my_identifier").unwrap();
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct SequenceViolin<T>
where
    T: Display + Clone,
{
    data: Vec<Vec<T>>,
    config: crate::configuration::Configuration,
}

impl<T> SequenceViolin<T>
where
    T: Display + Clone,
{
    /// Constructs a new ``SequenceViolin<T>``.
    ///
    /// # Examples
    ///
    /// From a complicated computation.
    /// ```
    /// use preexplorer::prelude::*;
    /// let data = (0..10).map(|i| i..10 + i);
    /// let seq = pre::SequenceViolin::new(data);
    /// ```
    pub fn new<I, J>(data: I) -> SequenceViolin<T>
    where
        I: IntoIterator<Item = J>,
        J: IntoIterator<Item = T>,
    {
        let data: Vec<Vec<T>> = data.into_iter().map(|j| j.into_iter().collect()).collect();
        let config = crate::configuration::Configuration::default();

        SequenceViolin { data, config }
    }
}

impl<T> Add for SequenceViolin<T>
where
    T: Display + Clone,
{
    type Output = crate::SequenceViolins<T>;

    fn add(self, other: crate::SequenceViolin<T>) -> crate::SequenceViolins<T> {
        let mut cmp = self.into();
        cmp += other;
        cmp
    }
}

impl<T> Configurable for SequenceViolin<T>
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

impl<T> Saveable for SequenceViolin<T>
where
    T: Display + Clone,
{
    fn plotable_data(&self) -> String {
        // Initial warning
        if self.data.is_empty() {
            eprintln!("Warning: There is no data.");
        }

        let mut plotable_data = String::new();

        for (counter, values) in self.data.clone().into_iter().enumerate() {
            for value in values {
                plotable_data.push_str(&format!("{}\t{}\n", counter, value));
            }
            // Separate datasets
            plotable_data.push_str("\n\n");
        }

        plotable_data
    }
}

impl<T> Plotable for SequenceViolin<T>
where
    T: Display + Clone,
{
    fn plot_script(&self) -> String {
        let mut gnuplot_script = self.opening_plot_script();

        gnuplot_script += &format!("\
renormalize = 2
do for [i=0:{}] {{
    # Computing some values
    set table $_
    plot {:?} index i using 2:(1) smooth kdensity
    unset table
    renormalize = (renormalize < 2 * GPVAL_Y_MAX) ? 2 * GPVAL_Y_MAX : renormalize
    # Plotting a greater domain
    set table '{}'.'_partial_plot'.i
    x_min = (GPVAL_X_MIN < GPVAL_X_MIN - 5 * GPVAL_KDENSITY_BANDWIDTH)? GPVAL_X_MIN : GPVAL_X_MIN - 5 * GPVAL_KDENSITY_BANDWIDTH
    x_max = (GPVAL_X_MAX > GPVAL_X_MAX + 5 * GPVAL_KDENSITY_BANDWIDTH)? GPVAL_X_MAX : GPVAL_X_MAX + 5 * GPVAL_KDENSITY_BANDWIDTH
    set xrange [x_min:x_max]
    plot {:?} index i using 2:(1) smooth kdensity
    unset table
    # Clean the plotting
    unset xrange
    unset yrange
}}

# Plotting the violins
# Right side
plot for [i=0:{}] '{}'.'_partial_plot'.i using (i + $2/renormalize):1 with filledcurve x=i linecolor i
# Left side
replot for [i=0:{}] '{}'.'_partial_plot'.i using (i - $2/renormalize):1 with filledcurve x=i linecolor i
",
            self.data.len() - 1,
            self.data_path(),
            self.data_path().display(),
            self.data_path(),
            self.data.len() - 1,
            self.data_path().display(),
            self.data.len() - 1,
            self.data_path().display(),
        );
        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

impl<T> From<crate::Densities<T>> for SequenceViolin<T>
where
    T: Display + Clone,
{
    fn from(mut densities: crate::Densities<T>) -> Self {
        let data: Vec<Vec<T>> = (0..densities.data_set.len())
            .map(|i| densities.data_set[i].realizations.clone())
            .collect();
        let mut seq_vio = SequenceViolin::new(data);
        let config = seq_vio.configuration_mut();
        *config = densities.configuration_mut().clone();
        seq_vio
    }
}

///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_style() {
        let data = (0..2).map(|i| -> Vec<u64> { (0..4).map(|j| j + i).collect() });
        let mut seq = SequenceViolin::new(data);
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
        let mut densities: pre::Densities<u64> = pre::Densities::new(many_dens);
        densities.set_title("My title");
        let seq_err = pre::SequenceViolin::from(densities.clone());

        assert_eq!(seq_err.title(), densities.title());
    }
}

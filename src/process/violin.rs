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
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;
use core::ops::Add;

/// Compare various ``Sequence``s.
pub mod comparison;

pub use comparison::ProcessViolins;

/// Sequence of values.
#[derive(Debug, PartialEq, Clone)]
pub struct ProcessViolin<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    domain: Vec<T>,
    image: Vec<Vec<S>>,
    config: crate::configuration::Configuration,
}

impl<T, S> ProcessViolin<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    /// Create a new ``ProcessViolin``.
    ///
    /// # Examples
    ///
    /// From a complicated computation.
    /// ```
    /// use preexplorer::prelude::*;
    /// let data = (0..10).map(|i| (i..10 + i));
    /// let pro_vio = pre::ProcessViolin::new((0..10), data);
    /// ```
    pub fn new<I, J, K>(domain: I, image: J) -> ProcessViolin<T, S>
    where
        I: IntoIterator<Item = T>,
        J: IntoIterator<Item = K>,
        K: IntoIterator<Item = S>,
    {
        let domain: Vec<T> = domain.into_iter().collect();
        let image: Vec<Vec<S>> = image.into_iter().map(|j| j.into_iter().collect()).collect();
        let config = crate::configuration::Configuration::default();

        ProcessViolin {
            domain,
            image,
            config,
        }
    }
}

impl<T, S> Add for ProcessViolin<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    type Output = ProcessViolins<T, S>;

    fn add(self, other: crate::ProcessViolin<T, S>) -> ProcessViolins<T, S> {
        let mut cmp = self.into();
        cmp += other;
        cmp
    }
}

impl<T, S> Configurable for ProcessViolin<T, S>
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

impl<T, S> Saveable for ProcessViolin<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn plotable_data(&self) -> String {
        // Initial warning
        if self.domain.is_empty() {
            eprintln!("Warning: There is no data.");
        }
        
        let mut plotable_data = String::new();

        for (time, values) in self.domain.clone().into_iter().zip(self.image.clone()) {
            for value in values {
                plotable_data.push_str(&format!("{}\t{}\n", time, value));
            }
            // Separate datasets
            plotable_data.push_str("\n\n");
        }
        plotable_data
    }
}

impl<T, S> Plotable for ProcessViolin<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn plot_script(&self) -> String {
        let mut gnuplot_script = self.opening_plot_script();

        gnuplot_script += &format!("array TIMES[{}] = [", self.domain.len());
        for i in 0..self.domain.len() - 1 {
            gnuplot_script += &format!("{}, ", self.domain[i]);
        }
        gnuplot_script += &format!("{}]\n", self.domain[self.domain.len() - 1]); // Last time

        gnuplot_script += &format!("\
RENORMALIZE = 2
do for [i=0:{}] {{
    # Computing some values
    set table $_
    plot {:?} index i using 2:(1) smooth kdensity
    unset table
    RENORMALIZE = (RENORMALIZE < 2 * GPVAL_Y_MAX) ? 2 * GPVAL_Y_MAX : RENORMALIZE
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
set style fill transparent solid 0.5
# Right side
plot for [i=0:{}] '{}'.'_partial_plot'.i using (TIMES[i+1] + $2/RENORMALIZE):1 with filledcurve x=TIMES[i+1] linecolor i
# Left side
replot for [i=0:{}] '{}'.'_partial_plot'.i using (TIMES[i+1] - $2/RENORMALIZE):1 with filledcurve x=TIMES[i+1] linecolor i
",
            self.image.len() - 1,
            self.data_path(),
            self.data_path().display(),
            self.data_path(),
            self.image.len() - 1,
            self.data_path().display(),
            self.image.len() - 1,
            self.data_path().display(),
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
        let domain = 0..2;
        let image = (0..2).map(|i| -> Vec<u64> { (0..4).map(|j| j + i).collect() });
        let mut seq = ProcessViolin::new(domain, image);
        seq.set_style("points");

        assert_eq!(
            &crate::configuration::plot::style::Style::Points,
            seq.style()
        );
    }
}

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
// use core::ops::Add;
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;

// /// Compare various ``Sequence``s.
// pub mod comparison;

// pub use comparison::Sequences;

/// Sequence of values.
#[derive(Debug, PartialEq, Clone)]
pub struct ProcessBin<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    domain: Vec<T>,
    image: Vec<Vec<S>>,
    binwidth: f64,
    config: crate::configuration::Configuration,
}

impl<T, S> ProcessBin<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    /// Create a new ``ProcessBin``.
    ///
    /// # Examples
    ///
    /// From a complicated computation.
    /// ```
    /// use preexplorer::prelude::*;
    /// let data = (0..10).map(|i| i * i + 1);
    /// let seq = pre::ProcessBin::new((0..10), data);
    /// ```
    pub fn new<I, J, K, U>(domain: I, image: J, binwidth: U) -> ProcessBin<T, S>
    where
        I: IntoIterator<Item = T>,
        J: IntoIterator<Item = K>,
        K: IntoIterator<Item = S>,
        U: Into<f64>,
    {
        let domain: Vec<T> = domain.into_iter().collect();
        let image: Vec<Vec<S>> = image.into_iter().map(|j| j.into_iter().collect()).collect();
        let config = crate::configuration::Configuration::default();
        let binwidth: f64 = binwidth.into();

        ProcessBin {
            domain,
            image,
            binwidth,
            config,
        }
    }
}

// impl<T, S> Add for ProcessBin<T, S>  
// where
//     T: Display + Clone,
//     S: Display + Clone,
// {
//     type Output = crate::ProcessBines<T, S>;

//     fn add(self, other: crate::ProcessBin<T, S>) -> crate::ProcessBines<T, S> { 
//         let mut cmp = self.into();
//         cmp += other;
//         cmp
//     }
// }

impl<T, S> Configurable for ProcessBin<T, S>
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

impl<T, S> Saveable for ProcessBin<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn plotable_data(&self) -> String {
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

impl<T, S> Plotable for ProcessBin<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn plot_script(&self) -> String {
        let mut gnuplot_script = self.opening_plot_script();

        gnuplot_script += &format!("BINWIDTH = {}\n", self.binwidth);
        gnuplot_script += &format!("array TIMES[{}] = [", self.domain.len());
        for i in 0..self.domain.len() - 1 {
            gnuplot_script += &format!("{}, ", self.domain[i]);
        }
        gnuplot_script += &format!("{}]\n", self.domain[self.domain.len() - 1]); // Last time

        gnuplot_script += &format!("array DataPoints[{}]\n", self.image.len());
        for i in 1..=self.image.len() {
            gnuplot_script += &format!("DataPoints[{}] = {}\n", i, self.image[i-1].len());
        }
        gnuplot_script += &format!("\
# Plotting each histogram
do for [i=0:{}] {{
    set table '{}'.'partial_plot'.i
    plot {:?} index i using 2:(1. / DataPoints[i+1]) bins binwidth=BINWIDTH with boxes # reference: http://www.bersch.net/gnuplot-doc/plot.html#commands-plot-datafile-bins 
    unset table
}}
# Plotting the serie of histograms
set style fill solid 0.5
plot for [i=0:{}] '{}'.'partial_plot'.i using (TIMES[i]):1:(TIMES[i]):(TIMES[i]+$2):3:4 with boxxyerrorbars # using x:y:xlow:xhigh:ylow:yhigh
",
            self.image.len() - 1,
            self.data_path().display(),
            self.data_path(),
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
        let image = (0..2).map(|i| -> Vec<u64> {
            (0..4).map(|j| j + i).collect()
        });
        let binwidth = 1;
        let mut seq = ProcessBin::new(domain, image, binwidth);
        seq.set_style("points");

        assert_eq!(
            &crate::configuration::plot::style::Style::Points,
            seq.style()
        );
    }
}

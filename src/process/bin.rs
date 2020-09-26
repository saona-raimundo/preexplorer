// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;
use core::ops::Add;

pub mod comparison;

pub use comparison::ProcessBins;

/// Indexed collection of histograms.
///
/// # Examples
///
/// Quick plot.
/// ```no_run
/// use preexplorer::prelude::*;
/// let image = (0..10).map(|i| (i..10 + i));
/// let binwidth = 1;
/// pre::ProcessBin::new((2..12), image, binwidth).plot("my_identifier").unwrap();
/// ```
///
/// Compare [ProcessBin] structs.
/// ```no_run
/// use preexplorer::prelude::*;
/// let image = (0..10).map(|i| (i..10 + i));
/// let binwidth = 1;
/// pre::ProcessBins::new(vec![
///     pre::ProcessBin::new((2..12), image.clone(), binwidth),
///     pre::ProcessBin::new((2..12), image, binwidth),
///     ])
///     .plot("my_identifier").unwrap();
/// ```
///
/// [ProcessBin]: struct.ProcessBin.html
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
    /// Constructs a new ``ProcessBin<T, S>``.
    ///
    /// # Examples
    ///
    /// From a complicated computation.
    /// ```
    /// use preexplorer::prelude::*;
    /// let domain = (2..12);
    /// let image = domain.clone().map(|i| (i..10 + i));
    /// let binwidth = 1;
    /// let pro_bin = pre::ProcessBin::new(domain, image, binwidth);
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

impl<T, S> Add for ProcessBin<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    type Output = crate::ProcessBins<T, S>;

    fn add(self, other: crate::ProcessBin<T, S>) -> crate::ProcessBins<T, S> {
        let mut cmp = self.into();
        cmp += other;
        cmp
    }
}

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

        gnuplot_script += &format!("array DataPoints[{}] = [", self.image.len());
        for i in 0..self.image.len() - 1 {
            gnuplot_script += &format!("{}, ", self.image[i].len());
        }
        gnuplot_script += &format!("{}]\n", self.image[self.image.len() - 1].len()); // Last time

        gnuplot_script += &format!("\
# Plotting each histogram
do for [i=0:{}] {{
    set table '{}'.'partial_plot'.i
    WEIGTH = 1. / (DataPoints[i+1] * BINWIDTH)
    plot {:?} index i using 2:WEIGTH bins binwidth=BINWIDTH with boxes # reference: http://www.bersch.net/gnuplot-doc/plot.html#commands-plot-datafile-bins 
    unset table
}}
# Plotting the serie of histograms
set style fill transparent solid 0.5
plot for [i=0:{}] '{}'.'partial_plot'.i using (TIMES[i+1]):1:(TIMES[i+1]):(TIMES[i+1]+$2):3:4 with boxxyerrorbars # using x:y:xlow:xhigh:ylow:yhigh
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
        let image = (0..2).map(|i| -> Vec<u64> { (0..4).map(|j| j + i).collect() });
        let binwidth = 1;
        let mut seq = ProcessBin::new(domain, image, binwidth);
        seq.set_style("points");

        assert_eq!(
            &crate::configuration::plot::style::Style::Points,
            seq.style()
        );
    }
}

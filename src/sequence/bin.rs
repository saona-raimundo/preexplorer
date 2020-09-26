// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;
use core::ops::Add;

/// Compare various ``Sequence``s.
pub mod comparison;

pub use comparison::SequenceBins;

/// Sequence of histograms normalize to represent a probability density function through bins.
///
/// To renormalize the histograms (that cover a unit area by default),
/// change the renormalization constant in the gnuplot script.  
/// By trying out with gnuplot, you can find the perfect fit.
///
/// # Examples
///
/// Quick plot.
/// ```no_run
/// use preexplorer::prelude::*;
/// let data = (0..10).map(|i| (i..10 + i));
/// let binwidth = 0.5;
/// pre::SequenceBin::new(data, binwidth).plot("my_identifier").unwrap();
/// ```
///
/// Compare [SequenceBin] structs.
/// ```no_run
/// use preexplorer::prelude::*;
/// pre::SequenceBins::new(vec![
///     pre::SequenceBin::new((0..10).map(|i| (i..10 + i)), 1),
///     pre::SequenceBin::new((0..10).map(|i| (i..10 + i)), 0.5),
///     ])
///     .plot("my_identifier").unwrap();
/// ```
///
/// [SequenceBin]: struct.SequenceBin.html
#[derive(Debug, PartialEq, Clone)]
pub struct SequenceBin<T>
where
    T: Display + Clone,
{
    data: Vec<Vec<T>>,
    binwidth: f64,
    config: crate::configuration::Configuration,
}

impl<T> SequenceBin<T>
where
    T: Display + Clone,
{
    /// Constructs a new ``SequenceBin<T>``.
    ///
    /// # Remarks
    ///
    /// To change the binwidth, please refer to the gnuplot script generated.
    /// By construction, a fixed binwidth is needed. This is okay in most of the cases, since it gives consistency and
    /// allows plotting constant values. If you want to change it, please go to the gnuplot script.
    /// 
    /// Negative binwidths are handled by gnuplot. 
    /// This crate simply prints the binwidth in the correct place in the gnuplot script generated, so
    /// if you use a value less or equal to zero gnuplot will resolve this issue by using a default behaviour.
    ///
    /// # Examples
    ///
    /// From a complicated computation.
    /// ```
    /// use preexplorer::prelude::*;
    /// let data = (0..10).map(|i| (i..10 + i));
    /// let binwidth = 0.5;
    /// let seq_bin = pre::SequenceBin::new(data, binwidth);
    /// ```
    pub fn new<I, J, S>(data: I, binwidth: S) -> SequenceBin<T>
    where
        I: IntoIterator<Item = J>,
        J: IntoIterator<Item = T>,
        S: Into<f64>,
    {
        let data: Vec<Vec<T>> = data.into_iter().map(|j| j.into_iter().collect()).collect();
        let config = crate::configuration::Configuration::default();
        let binwidth: f64 = binwidth.into();

        SequenceBin {
            data,
            binwidth,
            config,
        }
    }
}

impl<T> Add for SequenceBin<T>
where
    T: Display + Clone,
{
    type Output = crate::SequenceBins<T>;

    fn add(self, other: crate::SequenceBin<T>) -> crate::SequenceBins<T> {
        let mut cmp = self.into();
        cmp += other;
        cmp
    }
}

impl<T> Configurable for SequenceBin<T>
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

impl<T> Saveable for SequenceBin<T>
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

impl<T> Plotable for SequenceBin<T>
where
    T: Display + Clone,
{
    fn plot_script(&self) -> String {
        let mut gnuplot_script = self.opening_plot_script();

        gnuplot_script += &format!("BINWIDTH = {}\n", self.binwidth);
        gnuplot_script += &format!("array DataPoints[{}] = [", self.data.len());
        for i in 0..self.data.len() - 1 {
            gnuplot_script += &format!("{}, ", self.data[i].len());
        }
        gnuplot_script += &format!("{}]\n", self.data[self.data.len() - 1].len());

        gnuplot_script += &format!("\
# Plotting each histogram
do for [i=0:{}] {{
    set table '{}'.'partial_plot'.i
    plot {:?} index i using 2:(1. / (DataPoints[i+1] * {})) bins binwidth=BINWIDTH with boxes # reference: http://www.bersch.net/gnuplot-doc/plot.html#commands-plot-datafile-bins 
    unset table
}}
# Plotting the serie of histograms
set style fill transparent solid 0.5
plot for [i=0:{}] '{}'.'partial_plot'.i using (i):1:(i):(i+$2):3:4 with boxxyerrorbars # using x:y:xlow:xhigh:ylow:yhigh
",
            self.data.len() - 1,
            self.data_path().display(),
            self.data_path(),
            self.binwidth,
            self.data.len() - 1,
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
        let data = (0..2).map(|i| -> Vec<u64> { (0..4).map(|j| j + i).collect() });
        let binwidth = 1;
        let mut seq = SequenceBin::new(data, binwidth);
        seq.set_style("points");

        assert_eq!(
            &crate::configuration::plot::style::Style::Points,
            seq.style()
        );
    }
}

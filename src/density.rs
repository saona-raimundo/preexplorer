// Structs


// Traits
pub use crate::traits::Preexplorable;
use core::fmt::Display;

// Constants
use crate::{DATA_DIR_GNUPLOT};

pub use comparison::Comparison;

/// Compare various ``Distribution`` types together.
pub mod comparison;
/// Distribution with values with n-dimensions.
mod nddistribution;



/// Iterator over the data to be consumed when saved or plotted.
/// Can also be compared with other Distribution types.
///
/// # Examples
///
/// ```no_run
/// ```
///
/// # Remarks
///
/// See ``compare`` method to compare two or more data sets.
///
#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Density<I>
where
    I: IntoIterator + Clone,
    I::Item: PartialOrd + Display + Copy,
{
    pub(crate) realizations: I,
    pub(crate) config: crate::configuration::Configuration,
}

impl<I> Density<I>
where
    I: IntoIterator + Clone,
    I::Item: PartialOrd + Display + Copy,
{
    pub fn new(realizations: I) -> Density<I> {
        let mut config = crate::configuration::Configuration::default();
        config.style(crate::configuration::plot::style::Style::Steps);

        Density {
            realizations,
            config,
        }
    }

    /// Compare various ``Density`` types together.
    ///
    /// You can either put all together in a vector, or add them to a ``Comparison``
    ///
    /// # Remarks
    ///
    /// Titles of ``Density`` types involved in a ``Comparison`` are presented as legend.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// ```

    pub fn compare_with<J>(self, anothers: J) -> crate::density::comparison::Comparison<I>
    where
        J: IntoIterator<Item = crate::density::Density<I>>,
    {
        let mut comp = crate::density::comparison::Comparison::new(vec![self]);
        comp.add(anothers.into_iter());
        comp
    }
}

impl<I> crate::traits::Preexplorable for Density<I>
where
    I: IntoIterator + Clone,
    I::Item: PartialOrd + Display + Copy,
{
    /// Saves the data under ``data`` directory, and writes a basic plot_script to be used after execution.
    ///
    /// # Remark
    ///
    /// It is inteded for when one only wants to save the data, and not call any plotting
    /// during the Rust program execution. Posterior plotting can easily be done with the
    /// quick template gnuplot script saved under ``plots`` directory.
    fn raw_data(&self) -> String {

        let mut raw_data = String::new();
        for value in self.realizations.clone() {
            raw_data.push_str(&format!("{}\n", value));
        }
        raw_data
    }

    /// Write simple gnuplot script for this type of data.
    ///
    /// # Remark
    ///
    /// Only works for real numbers.
    fn plot_script(&self) -> String {

        let mut gnuplot_script = self.base_plot_script();

        // Values for the histogram

        let n = 20;
        let (mut min, mut max, mut length);
        length = 0;

        let mut realizations = self.realizations.clone().into_iter();
        match realizations.next() {
            Some(value) => {
                min = value;
                max = value;
                length += 1;
                for val in realizations {
                    // let val = val.into();
                    if val < min {
                        min = val;
                    }
                    if val > max {
                        max = val;
                    }
                    length += 1;
                }

                // Gnuplot scrpit

                gnuplot_script +=
                    "# Warning: this script only works when the data are real numbers. \n\n";

                gnuplot_script += &format!("nbins = {}.0 #number of bins\n", n);
                gnuplot_script += &format!("max = {} #max value\n", max);
                gnuplot_script += &format!("min = {} #min value\n", min);
                gnuplot_script += &format!("len = {}.0 #number of values\n", length);
                gnuplot_script += &format!("width = ({} - {}) / nbins #width\n\n", max, min);
                gnuplot_script += "# function used to map a value to the intervals\n";
                gnuplot_script += "hist(x,width) = width * floor(x/width) + width / 2.0\n\n";
                let dashtype = match self.get_dashtype() {
                    Some(dashtype) => dashtype,
                    None => 1,
                };
                gnuplot_script += &format!(
                    "plot \"{}/{}.txt\" using (hist($1,width)):(1.0/len) smooth frequency with {} dashtype {}\n",
                    DATA_DIR_GNUPLOT,
                    self.get_checked_id(),
                    self.get_style(),
                    dashtype,
                );
            }
            None => {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "No data to plot: There are no realizations, so no script can be prepared.",
                );
            }
        }

        // Gnuplot section

        gnuplot_script += "pause -1\n";

        gnuplot_script
    }

    fn configuration(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration_as_ref(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

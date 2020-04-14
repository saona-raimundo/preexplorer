// Structs


// Traits
pub use crate::traits::{Configurable, Saveable, Plotable, Comparison};
use core::fmt::Display;

// Structs
pub use comparison::Densities;

/// Compare various ``Distribution`` types together.
pub mod comparison;



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
#[derive(Debug, PartialEq, Clone)]
pub struct Density<T>
where
    T: PartialOrd + Display + Copy,
{
    pub(crate) realizations: Vec<T>,
    pub(crate) config: crate::configuration::Configuration,
}

impl<T> Density<T>
where
    T: PartialOrd + Display + Copy,
{
    pub fn new<I>(realizations: I) -> Density<T> 
    where
        I: IntoIterator<Item = T>,
    {
        let realizations: Vec<T> = realizations.into_iter().collect();
        let mut config = crate::configuration::Configuration::default();
        config.style(crate::configuration::plot::style::Style::Histeps);

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
    pub fn compare_with<J>(self, others: J) -> crate::density::comparison::Densities<T>
    where
        J: IntoIterator<Item = crate::density::Density<T>>,
    {
        let mut comp: Densities<T> = self.into();
        comp.add_many(others);
        comp
    }

    pub fn to_comparison(self) -> crate::density::comparison::Densities<T> {
        self.into()
    }
}

impl<T> Configurable for Density<T>
where
    T: PartialOrd + Display + Copy,
{
    fn configuration(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration_as_ref(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

impl<T> Saveable for Density<T>
where
    T: PartialOrd + Display + Copy,
{
    /// Saves the data under ``data`` directory, and writes a basic plot_script to be used after execution.
    ///
    /// # Remark
    ///
    /// It is inteded for when one only wants to save the data, and not call any plotting
    /// during the Rust program execution. Posterior plotting can easily be done with the
    /// quick template gnuplot script saved under ``plots`` directory.
    fn plotable_data(&self) -> String {

        let mut raw_data = String::new();
        for value in self.realizations.clone() {
            raw_data.push_str(&format!("{}\n", value));
        }
        raw_data
    }
}

impl<T> Plotable for Density<T>
where
    T: PartialOrd + Display + Copy,
{
    /// Write simple gnuplot script for this type of data.
    ///
    /// # Remark
    ///
    /// Only works for real numbers.
    fn plot_script(&self) -> String {

        let mut gnuplot_script = self.opening_plot_script();

        gnuplot_script += "set zeroaxis\n";
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
                gnuplot_script += "hist(x,width) = width * floor(x/width)\n\n";
                let dashtype = match self.get_dashtype() {
                    Some(dashtype) => dashtype,
                    None => 1,
                };
                gnuplot_script += &format!(
                    "plot {:?} using 1:(0.25*rand(0)-.35), \\\n",
                    self.get_data_path(),
                );
                gnuplot_script += &format!(
                    "\t \"\" using (hist($1,width)):(1./(width*len)) smooth frequency with {} dashtype {}, \\\n",
                    self.get_style(),
                    dashtype,
                );
                gnuplot_script += "\t \"\" using 1:(1.) smooth cnorm \n";
            }
            None => {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "No data to plot: There are no realizations, so no script can be prepared.",
                );
            }
        }

        // Gnuplot section

        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

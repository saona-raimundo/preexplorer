// Structs
use crate::errors::SavingError;

// Traits
pub use crate::traits::{Configurable, Saveable, Plotable};
use core::fmt::Display;

// Constants
use crate::{DATA_DIR_GNUPLOT};

/// See ``Density`` documentation for further use.
///
#[derive(Debug, PartialEq)]
pub struct Densities<I>
where
    I: IntoIterator + Clone,
    I::Item: PartialOrd + Display + Copy,
{
    pub(crate) data_set: Vec<crate::density::Density<I>>,
    pub(crate) config: crate::configuration::Configuration,
}

impl<I> Densities<I>
where
    I: IntoIterator + Clone,
    I::Item: PartialOrd + Display + Copy,
{
    pub fn new<K>(data_set: K) -> Densities<I>
    where
        K: IntoIterator<Item = crate::density::Density<I>>,
    {
        let config = crate::configuration::Configuration::default();
        let data_set = data_set
            .into_iter()
            .collect::<Vec<crate::density::Density<I>>>();
        Densities { data_set, config }
    }
}

impl<I> From<crate::density::Density<I>> for Densities<I> 
where
    I: IntoIterator + Clone,
    I::Item: PartialOrd + Display + Copy,
{
    fn from(density: crate::density::Density<I>) -> Self { 
        Densities::new(vec![density]) 
    }
}

impl<I> crate::traits::Comparison<crate::density::Density<I>> for Densities<I>
where
    I: IntoIterator + Clone,
    I::Item: PartialOrd + Display + Copy,
    {
    fn add(&mut self, other: crate::density::Density<I>) -> &mut Self {
        self.data_set.push(other);
        self
    }
}


impl<I> Configurable for Densities<I>
where
    I: IntoIterator + Clone,
    I::Item: PartialOrd + Display + Copy,
{
    fn configuration(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration_as_ref(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

impl<I> Saveable for Densities<I>
where
    I: IntoIterator + Clone,
    I::Item: PartialOrd + Display + Copy,
{
    fn raw_data(&self) -> String {
        let mut raw_data = String::new();
        for density in self.data_set.iter() {
            raw_data += &density.raw_data();
            raw_data += "\n";
        }
        raw_data
    }

    /// Saves the data under ``data`` directory, and writes a basic plot_script to be used after execution.
    ///
    /// # Remark
    ///
    /// It is inteded for when one only wants to save the data, and not call any plotting
    /// during the Rust program execution. Posterior plotting can easily be done with the
    /// quick template gnuplot script saved under ``plots`` directory.
    fn save_with_id(&self, id: &String) -> Result<&Self, SavingError> {
        for (counter, density) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            density.save_with_id(&inner_id)?;
        }

        Ok(self)
    }
}

impl<I> Plotable for Densities<I>
where
    I: IntoIterator + Clone,
    I::Item: PartialOrd + Display + Copy,
{
    /// Write simple gnuplot script for this type of data.
    ///
    fn plot_script(&self) -> String {

        let id = self.get_checked_id();
        let mut gnuplot_script = self.config.base_plot_script_comparison();

        // Treat each data to a prob distr funct

        for (counter, density) in self.data_set.iter().enumerate() {

            // Values for the histogram

            let n = 20;
            let (mut min, mut max, mut length);
            length = 0;

            let mut realizations = density.realizations.clone().into_iter();
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

                    // Gnuplot section

                    gnuplot_script += &format!("nbins_{} = {}.0 #number of bins\n", counter, n);
                    gnuplot_script += &format!("max_{} = {} #max value\n", counter, max);
                    gnuplot_script += &format!("min_{} = {} #min value\n", counter, min);
                    gnuplot_script += &format!("len_{} = {}.0 #number of values\n", counter, length);
                    gnuplot_script +=
                        &format!("width_{} = ({} - {}) / nbins_{} #width\n\n", counter, max, min, counter);
                    gnuplot_script += "# function used to map a value to the intervals\n";
                    gnuplot_script += &format!(
                        "hist_{}(x,width_{}) = width_{} * floor(x/width_{}) + width_{} / 2.0\n\n",
                        counter, counter, counter, counter, counter
                    );
                }
                None => {
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "No data to plot: There are no realizations, so no script can be prepared.",
                    );
                }
            }
        }

        gnuplot_script += "plot ";
        let style = self.get_style();
        let mut dashtype_counter = 0;

        for (counter, density) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            let legend = match density.get_title() {
                Some(leg) => String::from(leg),
                None => counter.to_string(),
            };
            let distribution_style = match style {
                crate::configuration::plot::style::Style::Default => density.get_style(),
                _ => style,
            };
            let dashtype = match density.get_dashtype() {
                Some(dashtype) => dashtype,
                None => {
                    dashtype_counter += 1;
                    dashtype_counter
                }
            };

            gnuplot_script += &format!(
                "\"{}/{}.txt\" using (hist_{}($1,width_{})):(1.0/len_{}) smooth frequency with {} title \"{}\" dashtype {}, ",
                DATA_DIR_GNUPLOT,
                inner_id,
                counter,
                counter,
                counter,
                distribution_style,
                legend,
                dashtype,
            );
            if counter < self.data_set.len() - 1 {
                gnuplot_script += "\\\n";
            }
        }
        gnuplot_script += "\n";
        gnuplot_script += "pause -1\n";

        gnuplot_script
    }
}

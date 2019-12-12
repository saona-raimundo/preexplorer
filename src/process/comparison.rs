// Structs
use crate::errors::SavingError;

// Traits
pub use crate::traits::Preexplorable;
use core::fmt::Display;

// Constants
use crate::{DATA_DIR_GNUPLOT};

/// See ``Process`` documentation for further use.
///
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Comparison<I, J>
where
    I: IntoIterator + Clone,
    I::Item: Display,
    J: IntoIterator + Clone,
    J::Item: Display,
{
    pub(crate) data_set: Vec<crate::process::Process<I, J>>,
    pub(crate) config: crate::configuration::Configuration,
}

impl<I, J> Comparison<I, J>
where
    I: IntoIterator + Clone,
    I::Item: Display,
    J: IntoIterator + Clone,
    J::Item: Display,
{
    pub fn new<K>(data_set: K) -> Comparison<I, J>
    where
        K: IntoIterator<Item = crate::process::Process<I, J>>,
    {
        let config = crate::configuration::Configuration::default();
        let data_set = data_set
            .into_iter()
            .collect::<Vec<crate::process::Process<I, J>>>();
        Comparison { data_set, config }
    }

    pub fn add<K>(&mut self, anothers: K)
    where
        K: IntoIterator<Item = crate::process::Process<I, J>>,
    {
        for process in anothers.into_iter() {
            self.data_set.push(process);
        }
    }
}

impl<I, J> crate::traits::Preexplorable for Comparison<I, J>
where
    I: IntoIterator + Clone,
    I::Item: Display,
    J: IntoIterator + Clone,
    J::Item: Display,
{
    fn raw_data(&self) -> String {
        let mut raw_data = String::new();
        for process in self.data_set.iter() {
            raw_data += &process.raw_data();
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
        for (counter, process) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            process.save_with_id(&inner_id)?;
        }
        Ok(self)
    }

    /// Write simple gnuplot script for this type of data.
    ///
    fn plot_script(&self) -> String {

        let id = self.get_checked_id();
        let mut gnuplot_script = self.config.base_plot_script_comparison();

        gnuplot_script += "plot ";
        let style = self.get_style();
        let mut dashtype_counter = 0;

        for (counter, process) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            let legend = match process.get_title() {
                Some(leg) => String::from(leg),
                None => counter.to_string(),
            };
            let process_style = match style {
                crate::configuration::plot::style::Style::Default => process.get_style(),
                _ => style,
            };
            let dashtype = match self.get_dashtype() {
                Some(dashtype) => dashtype,
                None => {
                    dashtype_counter += 1;
                    dashtype_counter
                }
            };

            gnuplot_script += &format!(
                "\"{}/{}.txt\" using 1:2 with {} title \"{}\" dashtype {}, ",
                DATA_DIR_GNUPLOT, inner_id, process_style, legend, dashtype,
            );
            if counter < self.data_set.len() - 1 {
                gnuplot_script += "\\\n";
            }
        }
        gnuplot_script += "\n";
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

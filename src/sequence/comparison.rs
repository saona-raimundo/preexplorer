// Structs
use crate::errors::SavingError;

// Traits
pub use crate::traits::{Configurable, Saveable, Plotable};
use core::fmt::Display;

// Constants
use crate::{DATA_DIR_GNUPLOT};

/// See ``Sequence`` documentation for further use.
///
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Comparison<I>
where
    I: IntoIterator + Clone,
    I::Item: Display,
{
    pub(crate) data_set: Vec<crate::sequence::Sequence<I>>,
    pub(crate) config: crate::configuration::Configuration,
}
impl<I> Comparison<I>
where
    I: IntoIterator + Clone,
    I::Item: Display,
{
    pub fn new<K>(data_set: K) -> Comparison<I>
    where
        K: IntoIterator<Item = crate::sequence::Sequence<I>>,
    {
        let config = crate::configuration::Configuration::default();
        let data_set = data_set
            .into_iter()
            .collect::<Vec<crate::sequence::Sequence<I>>>();
        Comparison { data_set, config }
    }

    pub fn add<J>(&mut self, anothers: J)
    where
        J: IntoIterator<Item = crate::sequence::Sequence<I>>,
    {
        for sequence in anothers.into_iter() {
            self.data_set.push(sequence);
        }
    }
}

impl<I> Configurable for Comparison<I>
where
    I: IntoIterator + Clone,
    I::Item: Display,
{
    fn configuration(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration_as_ref(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

impl<I> Saveable for Comparison<I>
where
    I: IntoIterator + Clone,
    I::Item: Display,
{
    fn raw_data(&self) -> String {
        let mut raw_data = String::new();
        for sequence in self.data_set.iter() {
            raw_data += &sequence.raw_data();
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

        for (counter, sequence) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            sequence.save_with_id(&inner_id)?;
        }

        Ok(self)
    }
}

impl<I> Plotable for Comparison<I>
where
    I: IntoIterator + Clone,
    I::Item: Display,
{
    /// Write simple gnuplot script for this type of data.
    ///
    fn plot_script(&self) -> String {

        let id = self.get_checked_id();
        let mut gnuplot_script = self.config.base_plot_script_comparison();

        gnuplot_script += "plot ";

        let style = self.get_style();
        let mut dashtype_counter = 0;

        for (counter, sequence) in self.data_set.iter().enumerate()  {
            let inner_id = format!("{}_{}", id, counter);
            let legend = match sequence.get_title() {
                Some(leg) => String::from(leg),
                None => counter.to_string(),
            };
            let sequence_style = match style {
                crate::configuration::plot::style::Style::Default => sequence.get_style(),
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
                DATA_DIR_GNUPLOT, inner_id, sequence_style, legend, dashtype
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

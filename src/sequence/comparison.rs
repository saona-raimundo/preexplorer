// Structs
use crate::errors::SavingError;

// Traits
pub use crate::traits::{Configurable, Saveable, Plotable};
use core::fmt::Display;

/// See ``Sequence`` documentation for further use.
///
#[derive(Debug, PartialEq)]
pub struct Sequences<T>
where
    T: Display,
{
    pub(crate) data_set: Vec<crate::sequence::Sequence<T>>,
    pub(crate) config: crate::configuration::Configuration,
}
impl<T> Sequences<T>
where
    T: Display,
{
    pub fn new<I>(data_set: I) -> Sequences<T>
    where
        I: IntoIterator<Item = crate::sequence::Sequence<T>>,
    {
        let config = crate::configuration::Configuration::default();
        let data_set = data_set
            .into_iter()
            .collect::<Vec<crate::sequence::Sequence<T>>>();
        Sequences { data_set, config }
    }
}

impl<T> From<crate::sequence::Sequence<T>> for Sequences<T> 
where
    T: Display,
{
    fn from(sequence: crate::sequence::Sequence<T>) -> Self { 
        Sequences::new(vec![sequence]) 
    }
}

impl<T> crate::traits::Comparison<crate::sequence::Sequence<T>> for Sequences<T>
where
    T: Display,
    {
    fn add(&mut self, other: crate::sequence::Sequence<T>) -> &mut Self {
        self.data_set.push(other);
        self
    }
}

impl<T> Configurable for Sequences<T>
where
    T: Display,
{
    fn configuration(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration_as_ref(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

impl<T> Saveable for Sequences<T>
where
    T: Display + Clone,
{
    fn plotable_data(&self) -> String {
        let mut raw_data = String::new();
        for sequence in self.data_set.iter() {
            raw_data += &sequence.plotable_data();
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

impl<T> Plotable for Sequences<T>
where
    T: Display + Clone,
{
    /// Write simple gnuplot script for this type of data.
    ///
    fn plot_script(&self) -> String {

        let id = self.get_checked_id();
        let mut gnuplot_script = self.config.opening_plot_script_comparison();

        gnuplot_script += "plot ";

        let style = self.get_style();
        let mut dashtype_counter = 0;

        for (counter, sequence) in self.data_set.iter().enumerate()  {
            let inner_id = format!("{}_{}", id, counter);
            let mut inner_path = self.get_data_path().to_path_buf();
            if let Some(extension) = self.get_data_extension() {
                inner_path.set_file_name(&inner_id);
                inner_path.set_extension(extension);
            } else {
                inner_path.set_file_name(&id);
            }
            let legend = match sequence.get_title() {
                Some(leg) => String::from(leg),
                None => counter.to_string(),
            };
            let sequence_style = match style {
                crate::configuration::plot::style::Style::Default => sequence.get_style(),
                _ => style,
            };
            let dashtype = match sequence.get_dashtype() {
                Some(dashtype) => dashtype,
                None => {
                    dashtype_counter += 1;
                    dashtype_counter
                }
            };
            gnuplot_script += &format!(
                "{:?} using 1:2 with {} title \"{}\" dashtype {}, ",
                inner_path, sequence_style, legend, dashtype
            );
            if counter < self.data_set.len() - 1 {
                gnuplot_script += "\\\n";
            }
        }
        gnuplot_script += "\n";
        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

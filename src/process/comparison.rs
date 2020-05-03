//! Comparison of indexed sequences of values.
//!
//! # Examples
//!
//! Quick plot.
//! ```no_run
//! use preexplorer::prelude::*;
//! let many_pros = (0..5).map(|_| ((0..10), (0..10)).preexplore());
//! pre::Processes::new(many_pros).plot("my_identifier").unwrap();
//! ```
//!

// Structs
use crate::errors::PreexplorerError;

// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;

/// Comparison counter part of ``Process`` struct.
///
#[derive(Debug, PartialEq)]
pub struct Processes<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    data_set: Vec<crate::process::Process<T, S>>,
    config: crate::configuration::Configuration,
}

impl<T, S> Processes<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    pub fn new<I>(data_set: I) -> Processes<T, S>
    where
        I: IntoIterator<Item = crate::process::Process<T, S>>,
    {
        let config = crate::configuration::Configuration::default();
        let data_set = data_set
            .into_iter()
            .collect::<Vec<crate::process::Process<T, S>>>();
        Processes { data_set, config }
    }
}

impl<T, S> From<crate::process::Process<T, S>> for Processes<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn from(process: crate::process::Process<T, S>) -> Self {
        Processes::new(vec![process])
    }
}

impl<T, S> crate::traits::Comparison<crate::process::Process<T, S>> for Processes<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn add(&mut self, other: crate::process::Process<T, S>) -> &mut Self {
        self.data_set.push(other);
        self
    }
}

impl<T, S> Configurable for Processes<T, S>
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

impl<T, S> Plotable for Processes<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn plot_script(&self) -> String {
        let id = self.checked_id();
        let mut gnuplot_script = self.config.opening_plot_script_comparison();

        gnuplot_script += "plot ";
        let style = self.style();
        let mut dashtype_counter = 0;

        for (counter, process) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            let mut inner_path = self.data_path().to_path_buf();
            if let Some(extension) = self.data_extension() {
                inner_path.set_file_name(&inner_id);
                inner_path.set_extension(extension);
            } else {
                inner_path.set_file_name(&id);
            }
            let legend = match process.title() {
                Some(leg) => String::from(leg),
                None => counter.to_string(),
            };
            let process_style = match style {
                crate::configuration::plot::style::Style::Default => process.style(),
                _ => style,
            };
            let dashtype = match process.dashtype() {
                Some(dashtype) => dashtype,
                None => {
                    dashtype_counter += 1;
                    dashtype_counter
                }
            };

            gnuplot_script += &format!(
                "{:?} using 1:2 with {} title \"{}\" dashtype {}, ",
                inner_path, process_style, legend, dashtype,
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

impl<T, S> Saveable for Processes<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn plotable_data(&self) -> String {
        let mut raw_data = String::new();
        for process in self.data_set.iter() {
            raw_data += &process.plotable_data();
            raw_data += "\n";
        }
        raw_data
    }

    fn save_with_id<U: Display>(&self, id: U) -> Result<&Self, PreexplorerError> {
        for (counter, process) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            process.save_with_id(&inner_id)?;
        }
        Ok(self)
    }
}

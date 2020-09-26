// Structs
use crate::errors::PreexplorerError;

// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;
use core::ops::{Add, AddAssign};

/// Comparison counter part of [SequenceError] struct.
///
/// # Examples
///
/// Quick plot.
/// ```no_run
/// use preexplorer::prelude::*;
/// let many_seq_err = (0..5).map(|_| pre::SequenceError::new((0..10).map(|i| i..10 + i)));
/// pre::SequenceErrors::new(many_seq_err).plot("my_identifier").unwrap();
/// ```
///
/// [SequenceError]: struct.SequenceError.html
#[derive(Debug, PartialEq)]
pub struct SequenceErrors {
    data_set: Vec<crate::sequence::error::SequenceError>,
    config: crate::configuration::Configuration,
}
impl SequenceErrors {
    pub fn new<I>(data_set: I) -> SequenceErrors
    where
        I: IntoIterator<Item = crate::sequence::error::SequenceError>,
    {
        let config = crate::configuration::Configuration::default();
        let data_set = data_set
            .into_iter()
            .collect::<Vec<crate::sequence::error::SequenceError>>();
        SequenceErrors { data_set, config }
    }
}

impl From<crate::sequence::SequenceError> for SequenceErrors {
    fn from(sequence: crate::sequence::error::SequenceError) -> Self {
        SequenceErrors::new(vec![sequence])
    }
}

impl Add<crate::SequenceError> for SequenceErrors {
    type Output = Self;

    fn add(mut self, other: crate::SequenceError) -> Self {
        self += other;
        self
    }
}

impl Add for SequenceErrors {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl AddAssign<crate::SequenceError> for SequenceErrors {
    fn add_assign(&mut self, other: crate::SequenceError) {
        self.data_set.push(other);
    }
}

impl AddAssign for SequenceErrors {
    fn add_assign(&mut self, mut other: Self) {
        self.data_set.append(&mut other.data_set);
    }
}

impl Configurable for SequenceErrors {
    fn configuration_mut(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

impl Saveable for SequenceErrors {
    fn plotable_data(&self) -> String {
        let mut raw_data = String::new();
        for sequence in self.data_set.iter() {
            raw_data += &sequence.plotable_data();
            raw_data += "\n";
        }
        raw_data
    }

    fn save_with_id<S: Display>(&self, id: S) -> Result<&Self, PreexplorerError> {
        for (counter, sequence) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            sequence.save_with_id(&inner_id)?;
        }

        Ok(self)
    }
}

impl Plotable for SequenceErrors {
    fn plot_script(&self) -> String {
        let id = self.checked_id();
        let mut gnuplot_script = self.config.opening_plot_script_comparison();

        gnuplot_script += "plot ";

        let style = self.style();
        let mut dashtype_counter = 0;

        for (counter, sequence) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            let mut inner_path = self.data_path().to_path_buf();
            if let Some(extension) = self.data_extension() {
                inner_path.set_file_name(&inner_id);
                inner_path.set_extension(extension);
            } else {
                inner_path.set_file_name(&id);
            }
            let legend = match sequence.title() {
                Some(leg) => String::from(leg),
                None => counter.to_string(),
            };
            let sequence_style = match style {
                crate::configuration::plot::style::Style::Default => sequence.style(),
                _ => style,
            };
            let dashtype = match sequence.dashtype() {
                Some(dashtype) => dashtype,
                None => {
                    dashtype_counter += 1;
                    dashtype_counter
                }
            };
            gnuplot_script += &format!(
                "{:?} using 1:2 with {} title \"{}\" dashtype {}, \\\n{:?} using 1:2:3 with yerrorbars notitle, ",
                inner_path, sequence_style, legend, dashtype, inner_path
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

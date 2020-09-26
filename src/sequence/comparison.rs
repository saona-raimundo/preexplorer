// Structs
use crate::errors::PreexplorerError;

// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;
use core::ops::{Add, AddAssign};

/// Comparison counter part of [Sequence] struct.
///
/// # Examples
///
/// Quick plot.
/// ```no_run
/// use preexplorer::prelude::*;
/// let many_seqs = (0..5).map(|_| (0..10).preexplore());
/// pre::Sequences::new(many_seqs).plot("my_identifier").unwrap();
/// ```
///
/// [Sequence]: struct.Sequence.html
#[derive(Debug, PartialEq)]
pub struct Sequences<T>
where
    T: Display + Clone,
{
    data_set: Vec<crate::sequence::Sequence<T>>,
    config: crate::configuration::Configuration,
}
impl<T> Sequences<T>
where
    T: Display + Clone,
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
    T: Display + Clone,
{
    fn from(sequence: crate::sequence::Sequence<T>) -> Self {
        Sequences::new(vec![sequence])
    }
}

impl<T> Add<crate::Sequence<T>> for Sequences<T>
where
    T: Display + Clone,
{
    type Output = Self;

    fn add(mut self, other: crate::Sequence<T>) -> Self {
        self += other;
        self
    }
}

impl<T> Add for Sequences<T>
where
    T: Display + Clone,
{
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl<T> AddAssign<crate::Sequence<T>> for Sequences<T>
where
    T: Display + Clone,
{
    fn add_assign(&mut self, other: crate::Sequence<T>) {
        self.data_set.push(other);
    }
}

impl<T> AddAssign for Sequences<T>
where
    T: Display + Clone,
{
    fn add_assign(&mut self, mut other: Self) {
        self.data_set.append(&mut other.data_set);
    }
}

impl<T> Configurable for Sequences<T>
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

    fn save_with_id<S: Display>(&self, id: S) -> Result<&Self, PreexplorerError> {
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
                inner_path.set_file_name(&inner_id);
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

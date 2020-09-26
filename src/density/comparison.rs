//! Comparison of histograms.
//!
//! # Examples
//!
//! Quick plot.
//! ```no_run
//! use preexplorer::prelude::*;
//! let many_dens = (0..5).map(|_| pre::Density::new(0..10));
//! pre::Densities::new(many_dens).plot("my_identifier").unwrap();
//! ```
//!

// Structs
use crate::errors::PreexplorerError;

// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;
use core::ops::{Add, AddAssign};

/// Comparison counter part of ``Density`` struct.
///
#[derive(Debug, PartialEq, Clone)]
pub struct Densities<T>
where
    T: Display + Clone,
{
    pub(crate) data_set: Vec<crate::density::Density<T>>,
    config: crate::configuration::Configuration,
}

impl<T> Densities<T>
where
    T: Display + Clone,
{
    pub fn new<K>(data_set: K) -> Densities<T>
    where
        K: IntoIterator<Item = crate::density::Density<T>>,
    {
        let config = crate::configuration::Configuration::default();
        let data_set = data_set
            .into_iter()
            .collect::<Vec<crate::density::Density<T>>>();
        Densities { data_set, config }
    }
}

impl<T> From<crate::Density<T>> for Densities<T>
where
    T: Display + Clone,
{
    fn from(density: crate::density::Density<T>) -> Self {
        Densities::new(vec![density])
    }
}

impl<T> Add<crate::Density<T>> for Densities<T>
where
    T: Display + Clone,
{
    type Output = Self;

    fn add(mut self, other: crate::Density<T>) -> Self {
        self += other;
        self
    }
}

impl<T> Add for Densities<T>
where
    T: Display + Clone,
{
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl<T> AddAssign<crate::Density<T>> for Densities<T>
where
    T: Display + Clone,
{
    fn add_assign(&mut self, other: crate::Density<T>) {
        self.data_set.push(other);
    }
}

impl<T> AddAssign for Densities<T>
where
    T: Display + Clone,
{
    fn add_assign(&mut self, mut other: Self) {
        self.data_set.append(&mut other.data_set);
    }
}

impl<T> Configurable for Densities<T>
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

impl<T> Saveable for Densities<T>
where
    T: Display + Clone,
{
    fn plotable_data(&self) -> String {
        let mut raw_data = String::new();
        for density in self.data_set.iter() {
            raw_data += &density.plotable_data();
            raw_data += "\n";
        }
        raw_data
    }

    fn save_with_id<S: Display>(&self, id: S) -> Result<&Self, PreexplorerError> {
        for (counter, density) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            density.save_with_id(&inner_id)?;
        }

        Ok(self)
    }
}

impl<T> Plotable for Densities<T>
where
    T: Display + Clone,
{
    fn plot_script(&self) -> String {
        let id = self.checked_id();
        let mut gnuplot_script = self.config.opening_plot_script_comparison();

        // Treat each data to a probability distribution function
        gnuplot_script += "plot ";
        let style = self.style();
        let mut dashtype_counter = 0;

        for (counter, density) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            let mut inner_path = self.data_path().to_path_buf();
            if let Some(extension) = self.data_extension() {
                inner_path.set_file_name(&inner_id);
                inner_path.set_extension(extension);
            } else {
                inner_path.set_file_name(&inner_id);
            }
            let legend = match density.title() {
                Some(leg) => String::from(leg),
                None => counter.to_string(),
            };
            let distribution_style = match style {
                crate::configuration::plot::style::Style::Default => density.style(),
                _ => style,
            };
            let dashtype = match density.dashtype() {
                Some(dashtype) => dashtype,
                None => {
                    dashtype_counter += 1;
                    dashtype_counter
                }
            };

            gnuplot_script += &format!(
                "{:?} using 1:({}) smooth kdensity with {} title \"{}\" dashtype {}, ",
                inner_path,
                1. / density.realizations.len() as f64,
                distribution_style,
                legend,
                dashtype,
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

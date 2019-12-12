//!
//! ```math
//! X: A \subseteq{\mathbb{R}} \to \mathbb{R} \,.
//! ```
//!

// Structs


// Traits
pub use crate::traits::Preexplorable;
use core::fmt::Display;

// Constants
use crate::{DATA_DIR_GNUPLOT};



/// Compare various ``Process`` types together.
pub mod comparison;
/// Time-series with values in R^n.
mod ndprocess;

pub use comparison::Comparison;


/// Iterator over the data to be consumed when saved or plotted. Can also be compared with other ``Process`` types.
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
pub struct Process<I, J>
where
    I: IntoIterator + Clone,
    I::Item: Display,
    J: IntoIterator + Clone,
    J::Item: Display,
{
    pub(crate) domain: I,
    pub(crate) image: J,
    pub(crate) config: crate::configuration::Configuration,
}

impl<I, J> Process<I, J>
where
    I: IntoIterator + Clone,
    I::Item: Display,
    J: IntoIterator + Clone,
    J::Item: Display,
{
    pub fn new(domain: I, image: J) -> Process<I, J> {
        let config = crate::configuration::Configuration::default();

        Process {
            domain,
            image,
            config,
        }
    }

    pub(crate) fn from_raw(
        domain: I,
        image: J,
        config: crate::configuration::Configuration,
    ) -> Process<I, J> {
        Process {
            domain,
            image,
            config,
        }
    }

    /// Pending documentation.
    pub fn compare_with<K>(self, anothers: K) -> crate::process::comparison::Comparison<I, J>
    where
        K: IntoIterator<Item = crate::process::Process<I, J>>,
    {
        let mut comp = crate::process::comparison::Comparison::new(vec![self]);
        comp.add(anothers.into_iter());
        comp
    }
}

impl<I, J> crate::traits::Preexplorable for Process<I, J>
where
    I: IntoIterator + Clone,
    I::Item: Display,
    J: IntoIterator + Clone,
    J::Item: Display,
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
        for (time, value) in self.domain.clone().into_iter().zip(self.image.clone()) {
            raw_data.push_str(&format!("{}\t{}\n", time, value));
        }
        raw_data
    }

    /// Write simple gnuplot script for this type of data.
    ///
    fn plot_script(&self) -> String {

        let mut gnuplot_script = self.base_plot_script();

        let dashtype = match self.get_dashtype() {
            Some(dashtype) => dashtype,
            None => 1,
        };

        gnuplot_script += &format!(
            "plot \"{}/{}.txt\" using 1:2 with {} dashtype {}\n",
            DATA_DIR_GNUPLOT,
            self.get_checked_id(),
            self.get_style(),
            dashtype,
        );
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

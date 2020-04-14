//!
//! ```math
//! X: A \subseteq{\mathbb{R}} \to \mathbb{R} \,.
//! ```
//!

// Structs


// Traits
pub use crate::traits::{Configurable, Saveable, Plotable, Comparison};
use core::fmt::Display;


/// Compare various ``Process`` types together.
pub mod comparison;

pub use comparison::Processes;


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
#[derive(Debug, PartialEq, Clone)]
pub struct Process<T, S>
{
    pub(crate) domain: Vec<T>,
    pub(crate) image: Vec<S>,
    pub(crate) config: crate::configuration::Configuration,
}

impl<T, S> Process<T, S>
where
    T: Display,
    S: Display,
{
    pub fn new<I, J>(domain: I, image: J) -> Process<T, S> 
    where
        I: IntoIterator<Item = T>,
        J: IntoIterator<Item = S>,
    {
        let domain: Vec<T> = domain.into_iter().collect();
        let image: Vec<S> = image.into_iter().collect();
        let config = crate::configuration::Configuration::default();

        Process {
            domain,
            image,
            config,
        }
    }

    pub fn to_comparison(self) -> crate::process::comparison::Processes<T, S> {
        self.into()
    }

    /// Pending documentation.
    pub fn compare_with<K>(self, others: K) -> crate::process::comparison::Processes<T, S>
    where
        K: IntoIterator<Item = crate::process::Process<T, S>>,
    {
        let mut comp: Processes<T, S> = self.into();
        comp.add_many(others);
        comp
    }
}

impl<T, S> Configurable for Process<T, S> {
    fn configuration(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration_as_ref(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

impl<T, S> Saveable for Process<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
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
        for (time, value) in self.domain.clone().into_iter().zip(self.image.clone()) {
            raw_data.push_str(&format!("{}\t{}\n", time, value));
        }
        raw_data
    }
}

impl<T, S> Plotable for Process<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{

    /// Write simple gnuplot script for this type of data.
    ///
    fn plot_script(&self) -> String {

        let mut gnuplot_script = self.opening_plot_script();

        let dashtype = match self.get_dashtype() {
            Some(dashtype) => dashtype,
            None => 1,
        };

        gnuplot_script += &format!(
            "plot {:?} using 1:2 with {} dashtype {}\n",
            self.get_data_path(),
            self.get_style(),
            dashtype,
        );
        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }

    
}

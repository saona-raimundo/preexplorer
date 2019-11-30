//!
//! ```math
//! X: A \subseteq{\mathbb{R}} \to \mathbb{R} \,.
//! ```
//!

use crate::errors::SavingError;
pub use comparison::Comparison;

/// Compare various ``Process`` types together.
pub mod comparison;
/// Time-series with values in R^n.
pub mod ndprocess;

pub use crate::traits::Preexplorable;

// Trait bounds
use core::fmt::Display;

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
    fn save<S: Display>(&self, serie: S) -> Result<&Self, SavingError> {
        let serie = &serie.to_string();
        self.write_plot_script(serie)?;

        // Files creation

        let data_dir = "data";
        std::fs::create_dir_all(data_dir)?;

        let data_name = &format!("{}.{}", serie, self.get_extension());
        let path = &format!("{}\\{}", data_dir, data_name);

        // Create the data structure for gnuplot

        let mut data_gnuplot = String::new();
        if self.get_header() {
            data_gnuplot.push_str(&format!("# {}", serie));
            match self.get_title() {
                Some(title) => data_gnuplot.push_str(&format!(": {}\n", title)),
                None => data_gnuplot.push_str("\n"),
            }
            data_gnuplot.push_str("# time value\n");
        }
        for (time, value) in self.domain.clone().into_iter().zip(self.image.clone()) {
            data_gnuplot.push_str(&format!("{}\t{}\n", time, value));
        }

        // Write the data

        std::fs::write(path, data_gnuplot)?;

        Ok(self)
    }

    /// Plots the data by: saving it in hard-disk, writting a plot script for gnuplot and calling it.
    ///
    /// # Remark
    ///
    /// The plot will be executed asyncroniously and idependently of the Rust program.
    ///
    fn plot<S: Display>(&self, serie: S) -> Result<&Self, SavingError> {
        let serie = &serie.to_string();
        self.write_plot_script(serie)?;
        self.save(serie)?;

        let gnuplot_file = format!("{}.gnu", serie);

        let gnuplot_file = &format!("plots\\{}", gnuplot_file);
        std::process::Command::new("gnuplot")
            .arg(gnuplot_file)
            .spawn()?;
        Ok(self)
    }

    /// Write simple gnuplot script for this type of data.
    ///
    fn write_plot_script<S: Display>(&self, serie: S) -> Result<&Self, SavingError> {
        std::fs::create_dir_all("plots")?;
        let gnuplot_file = &format!("plots\\{}.gnu", serie);

        let mut gnuplot_script = self.base_plot_script();

        let dashtype = match self.get_dashtype() {
            Some(dashtype) => dashtype,
            None => 1,
        };

        gnuplot_script += &format!("plot \"data/{}.txt\" using 1:2 with {} dashtype {}\n", 
            serie, 
            self.get_style(),
            dashtype,
        );
        gnuplot_script += "pause -1\n";

        std::fs::write(&gnuplot_file, &gnuplot_script)?;

        Ok(self)
    }

    fn configuration(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration_as_ref(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

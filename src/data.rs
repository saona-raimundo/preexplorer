// Structs
use crate::errors::SavingError;

// Traits
pub use crate::traits::{Configurable, Saveable, Plotable};
use core::fmt::Display;

// Constants
use crate::{DATA_DIR_GNUPLOT, PLOT_DIR};

/// Missing documentation.
///
#[derive(Debug, PartialEq, Clone)]
pub struct Data<I>
where
    I: IntoIterator + Clone,
    I::Item: Display,
{
    pub(crate) data: I,
    pub(crate) config: crate::configuration::Configuration,
    pub(crate) dim: usize,
}

impl<I> Data<I>
where
    I: IntoIterator + Clone,
    I::Item: Display,
{
    pub fn new(data: I, dim: usize) -> Self {
        let config = crate::configuration::Configuration::default();
        Data { data, config, dim }
    }
}

impl<I> Configurable for Data<I>
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

impl<I> Saveable for Data<I>
where
    I: IntoIterator + Clone,
    I::Item: Display,
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

        let mut counter = 0;
        for value in self.data.clone() {
            raw_data.push_str(&format!("{}\t", value));
            counter += 1;
            if counter == self.dim {
                counter = 0;
                raw_data.push_str("\n");
            }
        }

        raw_data
    }
}

impl<I> Plotable for Data<I>
where
    I: IntoIterator + Clone,
    I::Item: Display,
{

    /// Plots the data by: saving it in hard-disk, writting a plot script for gnuplot and calling it.
    ///
    /// # Remark
    ///
    /// The plot will be executed asyncroniously and idependently of the Rust program.
    ///
    fn plot(&mut self, id: &str) -> Result<&mut Self, SavingError> {
        self.id(id);

        match self.dim {
    		1 => {
    			let mut sequence = crate::sequence::Sequence::from_raw(self.data.clone(), self.config.clone());
    			sequence.plot(&self.get_checked_id())?;
                Ok(self)
    		},
    		2 => {
    			// separate iterators
    			let mut first_filter = vec![true, false].into_iter().cycle();
    			let mut second_filter = vec![false, true].into_iter().cycle();

    			let first_data = self.data.clone()
                    .into_iter()
                    .filter(move |_| first_filter.next().unwrap())
                    .collect::<Vec<_>>();
    			let second_data = self.data.clone()
                    .into_iter()
                    .filter(move |_| second_filter.next().unwrap())
                    .collect::<Vec<_>>();

    			let mut process = crate::process::Process::from_raw(
                    first_data.iter(),
                    second_data.iter(),
                    self.config.clone());

    			process.plot(&self.get_checked_id())?;
                Ok(self)
    		},
    		_ => return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other, "ploting general data: dimension of data is too high to be automatically ploted. Please do it yourself."
                ).into()
            ),
    	}
    }

    /// Write simple gnuplot script for this type of data.
    ///
    fn plot_script(&self) -> String {
        match self.dim {
            1 => {
                let sequence =
                    crate::sequence::Sequence::from_raw(self.data.clone(), self.config.clone());
                sequence.plot_script()
            }
            2 => {
                // separate iterators
                let mut first_filter = vec![true, false].into_iter().cycle();
                let mut second_filter = vec![false, true].into_iter().cycle();

                let first_data = self
                    .data
                    .clone()
                    .into_iter()
                    .filter(move |_| first_filter.next().unwrap())
                    .collect::<Vec<_>>();
                let second_data = self
                    .data
                    .clone()
                    .into_iter()
                    .filter(move |_| second_filter.next().unwrap())
                    .collect::<Vec<_>>();

                let process = crate::process::Process::from_raw(
                    first_data.iter(),
                    second_data.iter(),
                    self.config.clone(),
                );

                process.plot_script()
            }
            _ => {

                let mut gnuplot_script = self.base_plot_script();

                gnuplot_script += "\n# Visit http://www.gnuplotting.org and search for the correct plotting command!\n";
                gnuplot_script += "\n# To get the plot, run the following command:";
                gnuplot_script += &format!("\n# gnuplot \"{}\\{}.gnu\" \n\n", PLOT_DIR, self.get_checked_id());
                gnuplot_script += &format!("plot \"{}/{}.txt\" \n", DATA_DIR_GNUPLOT, self.get_checked_id());
                gnuplot_script += "pause -1\n";

                gnuplot_script
            }
        }
    }
}

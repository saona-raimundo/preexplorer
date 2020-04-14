// Structs
use crate::errors::SavingError;

// Traits
pub use crate::traits::{Configurable, Saveable, Plotable};
use core::fmt::Display;

/// Missing documentation.
///
#[derive(Debug, PartialEq, Clone)]
pub struct Data<T>
where
    T: Display,
{
    pub(crate) data: Vec<T>,
    pub(crate) config: crate::configuration::Configuration,
    pub(crate) dim: usize,
}

impl<T> Data<T>
where
    T: Display,
{
    pub fn new<I>(data: I, dim: usize) -> Self 
    where
        I: IntoIterator<Item = T>,
    {
        let data: Vec<T> = data.into_iter().collect();
        let config = crate::configuration::Configuration::default();
        Data { data, config, dim }
    }
}

impl<T> Configurable for Data<T>
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

impl<T> Saveable for Data<T>
where
    T: Display + Clone,
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

impl<T> Plotable for Data<T>
where
    T: Display + Clone,
{

    /// Plots the data by: saving it in hard-disk, writting a plot script for gnuplot and calling it.
    ///
    /// # Remark
    ///
    /// The plot will be executed asyncroniously and idependently of the Rust program.
    ///
    fn plot(&mut self, id: &str) -> Result<&mut Self, SavingError> {
        
        self.plot_later(id)?;

        let message = format!("Tried to plot general data: do it directly with gnuplot. A preliminar gnuplot script is located in {:?}", self.get_plot_path());
        Err(std::io::Error::new(
            std::io::ErrorKind::Other, 
            message
        ).into())
    }

    /// Write simple gnuplot script for this type of data.
    ///
    fn plot_script(&self) -> String {

        let mut gnuplot_script = self.opening_plot_script();

        gnuplot_script += "\n# Visit http://www.gnuplotting.org and search for the correct plotting command!\n";
        gnuplot_script += "\n# To get the plot, run the following command:";
        gnuplot_script += &format!("\n# gnuplot {:?} \n\n", self.get_plot_path());
        gnuplot_script += &format!("plot {:?} \n", self.get_data_path());
        
        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

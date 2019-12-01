use crate::errors::SavingError;

pub use crate::traits::Preexplorable;

// Trait bounds
use core::fmt::Display;

/// Missing documentation.
///
#[derive(Debug, PartialOrd, PartialEq, Clone)]
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

impl<I> crate::traits::Preexplorable for Data<I>
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
    fn save<S: Display>(&self, serie: S) -> Result<&Self, SavingError> {
        let serie = &serie.to_string();
        self.write_plot_script(serie)?;

        // Files creation

        let data_dir = "preexplorer\\data";
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
            data_gnuplot.push_str("#");
            for i in 1..self.dim + 1 {
                data_gnuplot.push_str(&format!(" dim_{}", i));
            }
            data_gnuplot.push_str("\n");
        }

        let mut counter = 0;
        for value in self.data.clone() {
            data_gnuplot.push_str(&format!("{}\t", value));
            counter += 1;
            if counter == self.dim {
                counter = 0;
                data_gnuplot.push_str("\n");
            }
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
        match self.dim {
    		1 => {
    			let sequence = crate::sequence::Sequence::from_raw(self.data.clone(), self.config.clone());
    			sequence.plot(serie)?;
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

    			let process = crate::process::Process::from_raw(
                    first_data.iter(), 
                    second_data.iter(), 
                    self.config.clone());

    			process.plot(serie)?;
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
    fn write_plot_script<S: Display>(&self, serie: S) -> Result<&Self, SavingError> {
        match self.dim {
    		1 => {
    			let sequence = crate::sequence::Sequence::from_raw(self.data.clone(), self.config.clone());
    			sequence.write_plot_script(serie)?;
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

                let process = crate::process::Process::from_raw(
                    first_data.iter(), 
                    second_data.iter(), 
                    self.config.clone());
                
    			process.write_plot_script(serie)?;
                Ok(self)
    		},
    		_ => {
                std::fs::create_dir_all("preexplorer\\plots")?;
                let gnuplot_file = &format!("preexplorer\\plots\\{}.gnu", serie);

                let mut gnuplot_script = self.base_plot_script();

                let dashtype = match self.get_dashtype() {
                    Some(dashtype) => dashtype,
                    None => 1,
                };
                gnuplot_script += &format!("plot \"preexplorer/data/{}.txt\" with {} dashtype {} \n", 
                    serie,
                    self.get_style(),
                    dashtype,
                );
                gnuplot_script += "pause -1\n";

                std::fs::write(&gnuplot_file, &gnuplot_script)?;

                Ok(self)
            },
    	}
    }

    fn configuration(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }

    fn configuration_as_ref(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

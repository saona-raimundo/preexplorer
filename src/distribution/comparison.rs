use crate::errors::SavingError;
pub use crate::traits::Preexplorable;

// Trait bounds
use core::fmt::Display;

/// See ``Distribution`` documentation for further use.
///
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Comparison<I>
where
    I: ExactSizeIterator + Clone,
    I::Item: PartialOrd + Display + Copy,
{
    pub(crate) data_set: Vec<crate::distribution::Distribution<I>>,
    pub(crate) config: crate::configuration::Configuration,
}

impl<I> Comparison<I>
where
    I: ExactSizeIterator + Clone,
    I::Item: PartialOrd + Display + Copy,
{
    pub fn new<K>(data_set: K) -> Comparison<I>
    where
        K: IntoIterator<Item = crate::distribution::Distribution<I>>,
    {
        let config = crate::configuration::Configuration::default();
        let data_set = data_set
            .into_iter()
            .collect::<Vec<crate::distribution::Distribution<I>>>();
        Comparison { data_set, config }
    }

    pub fn add<J>(&mut self, anothers: J)
    where
        J: IntoIterator<Item = crate::distribution::Distribution<I>>,
    {
        for sequence in anothers.into_iter() {
            self.data_set.push(sequence);
        }
    }
}

impl<I> crate::traits::Preexplorable for Comparison<I>
where
    I: ExactSizeIterator + Clone,
    I::Item: PartialOrd + Display + Copy,
{
    /// Saves the data under ``data`` directory, and writes a basic plot_script to be used after execution.
    ///
    /// # Remark
    ///
    /// It is inteded for when one only wants to save the data, and not call any plotting
    /// during the Rust program execution. Posterior plotting can easily be done with the
    /// quick template gnuplot script saved under ``plots`` directory.
    fn save<S: Display>(&self, serie: S) -> Result<&Self, SavingError> {
        for (counter, distribution) in self.data_set.iter().enumerate() {
            crate::distribution::Distribution::save(
                &distribution,
                &format!("{}_{}", serie, counter),
            )?;
        }

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

        let mut gnuplot_script = self.config.base_plot_script();

        // Treat each data to a prob distr funct

        for i in 0..self.data_set.len() {
            let distribution = &self.data_set[i];

            // Values for the histogram

            let n = 20;
            let (mut min, mut max, mut length);
            length = 0;
            
            let mut realizations = distribution.realizations.clone();
            match realizations.next() {
                Some(value) => {
                    min = value;
                    max = value;
                    length += 1;
                    for val in realizations {
                        // let val = val.into();
                        if val < min { min = val; }
                        if val > max { max = val; }
                        length += 1;
                    }

                    // Gnuplot section

                    gnuplot_script += &format!("nbins_{} = {}.0 #number of bins\n", i, n);
                    gnuplot_script += &format!("max_{} = {} #max value\n", i, max);
                    gnuplot_script += &format!("min_{} = {} #min value\n", i, min);
                    gnuplot_script += &format!("len_{} = {}.0 #number of values\n", i, length);
                    gnuplot_script += &format!(
                        "width_{} = ({} - {}) / nbins_{} #width\n\n",
                        i,
                        max, min,
                        i
                    );
                    gnuplot_script += "# function used to map a value to the intervals\n";
                    gnuplot_script += &format!(
                        "hist_{}(x,width_{}) = width_{} * floor(x/width_{}) + width_{} / 2.0\n\n",
                        i, i, i, i, i
                    );
                },
                None => (),
            }    
        }

        gnuplot_script += "plot ";
        let style = self.style();
        for i in 0..self.data_set.len() {
            let distribution = &self.data_set[i];
            let legend = match distribution.config.title() {
                Some(leg) => String::from(leg),
                None => i.to_string(),
            };
            let distribution_style = match style {
                crate::configuration::plot::style::Style::Default => distribution.style(),
                _ => style,
            };
            gnuplot_script += &format!(
                "\"data/{}_{}.txt\" using (hist_{}($1,width_{})):(1.0/len_{}) smooth frequency with {} title \"{}\" dashtype {}, ",
                serie, 
                i, 
                i, 
                i, 
                i, 
                distribution_style,
                legend, 
                i+1,
            );
            if i < self.data_set.len() - 1 {
                gnuplot_script += "\\\n";
            }
        }
        gnuplot_script += "\n";
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

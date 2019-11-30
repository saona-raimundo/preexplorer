use crate::errors::SavingError;
pub use crate::traits::Preexplorable;

// Trait bounds
use core::fmt::Display;

/// See ``Sequence`` documentation for further use.
///
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Comparison<I>
where
    I: IntoIterator + Clone,
    I::Item: Display,
{
    pub(crate) data_set: Vec<crate::sequence::Sequence<I>>,
    pub(crate) config: crate::configuration::Configuration,
}
impl<I> Comparison<I>
where
    I: IntoIterator + Clone,
    I::Item: Display,
{
    pub fn new<K>(data_set: K) -> Comparison<I>
    where
        K: IntoIterator<Item = crate::sequence::Sequence<I>>,
    {
        let config = crate::configuration::Configuration::default();
        let data_set = data_set
            .into_iter()
            .collect::<Vec<crate::sequence::Sequence<I>>>();
        Comparison { data_set, config }
    }

    pub fn add<J>(&mut self, anothers: J)
    where
        J: IntoIterator<Item = crate::sequence::Sequence<I>>,
    {
        for sequence in anothers.into_iter() {
            self.data_set.push(sequence);
        }
    }
}

impl<I> crate::traits::Preexplorable for Comparison<I>
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
        for (counter, sequence) in self.data_set.iter().enumerate() {
            crate::sequence::Sequence::save(&sequence, &format!("{}_{}", serie, counter))?;
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

        let mut gnuplot_script = self.config.base_plot_script_comparison();

        gnuplot_script += "plot ";

        let style = self.get_style();
        let mut dashtype_counter = 0;

        for i in 0..self.data_set.len() {
            let sequence = &self.data_set[i];
            let legend = match sequence.get_title() {
                Some(leg) => String::from(leg),
                None => i.to_string(),
            };
            let sequence_style = match style {
                crate::configuration::plot::style::Style::Default => sequence.get_style(),
                _ => style,
            };
            let dashtype = match self.get_dashtype() {
                Some(dashtype) => dashtype,
                None => {
                    dashtype_counter += 1;
                    dashtype_counter
                },
            };
            gnuplot_script += &format!(
                "\"data/{}_{}.txt\" using 1:2 with {} title \"{}\" dashtype {}, ",
                serie,
                i,
                sequence_style,
                legend,
                dashtype
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

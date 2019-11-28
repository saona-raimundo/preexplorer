use crate::errors::SavingError;
pub use crate::traits::Preexplorable;

// Trait bounds
use core::fmt::Display;

/// See ``Sequence`` documentation for further use.
///
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Comparison<I>
where
    I: ExactSizeIterator + Clone,
    I::Item: Display,
{
    pub(crate) data_set: Vec<crate::sequence::Sequence<I>>,
    pub(crate) config: crate::configuration::Configuration,
}
impl<I> Comparison<I>
where
    I: ExactSizeIterator + Clone,
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
    I: ExactSizeIterator + Clone,
    I::Item: Display,
{
    /// Saves the data under ``data`` directory, and writes a basic plot_script to be used after execution.
    ///
    /// # Remark
    ///
    /// It is inteded for when one only wants to save the data, and not call any plotting
    /// during the Rust program execution. Posterior plotting can easily be done with the
    /// quick template gnuplot script saved under ``plots`` directory.
    fn save<S: Display>(&self, serie: &S) -> Result<&Self, SavingError> {
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
    fn plot<S: Display>(&self, serie: &S) -> Result<&Self, SavingError> {
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
    fn write_plot_script<S: Display>(&self, serie: &S) -> Result<&Self, SavingError> {
        std::fs::create_dir_all("plots")?;
        let gnuplot_file = &format!("plots\\{}.gnu", serie);

        let mut gnuplot_script = self.config.base_plot_script();

        gnuplot_script += "plot ";
        for i in 0..self.data_set.len() {
            let legend = match &self.data_set[i].config.title() {
                Some(leg) => String::from(leg),
                None => i.to_string(),
            };
            gnuplot_script += &format!(
                "\"data/{}_{}.txt\" using 1:2 with lines title \"{}\" dashtype {}, ",
                serie,
                i,
                legend,
                i + 1
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

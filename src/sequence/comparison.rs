use crate::errors::SavingError;
pub use crate::traits::PlotableStructure;

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
    pub(crate) options: crate::sequence::SequenceOptions,
}
impl<I> Comparison<I>
where
    I: IntoIterator + Clone,
    I::Item: Display,
{
    pub fn new(data_set: Vec<crate::sequence::Sequence<I>>) -> Comparison<I> {
        let options = crate::sequence::SequenceOptions::default();

        Comparison { data_set, options }
    }

    pub fn set_title<S: Display>(mut self, title: S) -> Self {
        self.options.set_title(title.to_string());
        self
    }
    pub fn set_logx(mut self, logx: f64) -> Self {
        self.options.set_logx(logx);
        self
    }
    pub fn set_logy(mut self, logy: f64) -> Self {
        self.options.set_logy(logy);
        self
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

impl<'a, I> crate::traits::PlotableStructure for Comparison<I>
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
    fn save<S: Display>(mut self, serie: &S) -> Result<(), SavingError> {
        for i in (0..self.data_set.len()).rev() {
            match self.data_set.pop() {
                Some(sequence) => {
                    crate::sequence::Sequence::save(sequence, &format!("{}_{}", serie, i))?
                }
                None => break,
            }
        }

        Ok(())
    }

    /// Plots the data by: saving it in hard-disk, writting a plot script for gnuplot and calling it.
    ///
    /// # Remark
    ///
    /// The plot will be executed asyncroniously and idependently of the Rust program.
    ///
    fn plot<S: Display>(self, serie: &S) -> Result<(), SavingError> {
        self.write_plot_script(serie)?;
        self.save(serie)?;

        let gnuplot_file = format!("{}.gnu", serie);

        let gnuplot_file = &format!("plots\\{}", gnuplot_file);
        std::process::Command::new("gnuplot")
            .arg(gnuplot_file)
            .spawn()?;
        Ok(())
    }

    /// Write simple gnuplot script for this type of data.
    ///
    fn write_plot_script<S: Display>(&self, serie: &S) -> Result<(), SavingError> {
        std::fs::create_dir_all("plots")?;
        let gnuplot_file = &format!("plots\\{}.gnu", serie);

        let mut gnuplot_script = String::new();
        gnuplot_script += "set key\n";
        if let Some(title) = &self.options.title {
            gnuplot_script += &format!("set title \"{}\"\n", title);
        }
        if let Some(logx) = &self.options.logx {
            if *logx <= 0.0 {
                gnuplot_script += "set logscale x\n";
            } else {
                gnuplot_script += &format!("set logscale x {}\n", logx);
            }
        }
        if let Some(logy) = &self.options.logy {
            if *logy <= 0.0 {
                gnuplot_script += "set logscale y\n";
            } else {
                gnuplot_script += &format!("set logscale y {}\n", logy);
            }
        }

        gnuplot_script += "plot ";
        for i in 0..self.data_set.len() {
            let legend = match &self.data_set[i].options.title {
                Some(leg) => String::from(leg),
                None => i.to_string(),
            };
            gnuplot_script += &format!(
                "\"data/{}_{}.txt\" using 1:2 with lines title \"{}\", ",
                serie, i, legend
            );
        }
        gnuplot_script += "\n";
        gnuplot_script += "pause -1\n";

        std::fs::write(&gnuplot_file, &gnuplot_script)?;

        Ok(())
    }
}

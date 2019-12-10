// Structs
use crate::errors::SavingError;

// Traits
pub use crate::traits::Preexplorable;
use core::fmt::Display;

// Constants
use crate::{PLOT_DIR, DATA_DIR_GNUPLOT};

/// Compare various ``Sequence`` types together.
pub mod comparison;
/// Sequence with values with n-dimensions.
mod ndsequence;
pub use comparison::Comparison;




/// Iterator over the data to be consumed when saved or plotted. Can also be compared with other Sequence types.
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
pub struct Sequence<I>
where
    I: IntoIterator + Clone,
    I::Item: Display,
{
    pub(crate) data: I,
    pub(crate) config: crate::configuration::Configuration,
}

impl<I> Sequence<I>
where
    I: IntoIterator + Clone,
    I::Item: Display,
{
    pub fn new(data: I) -> Sequence<I> {
        let config = crate::configuration::Configuration::default();

        Sequence { data, config }
    }

    pub(crate) fn from_raw(data: I, config: crate::configuration::Configuration) -> Sequence<I> {
        Sequence { data, config }
    }

    /// Compare various ``Sequence`` types together.
    ///
    /// You can either put all together in a vector, or add them to a ``Comparison``
    ///
    /// # Remarks
    ///
    /// Titles of ``Sequence`` types involved in a ``Comparison`` are presented as legend.
    ///
    /// # Examples
    ///
    /// Compare many ``Sequence`` types by gathering all first.
    ///
    /// ```no_run
    /// ```
    ///
    /// Compare some, keep computing, add to the comparison and then save/plot all together.
    ///
    /// ```no_run
    /// ```
    ///
    pub fn compare_with<J>(self, anothers: J) -> crate::sequence::comparison::Comparison<I>
    where
        J: IntoIterator<Item = crate::sequence::Sequence<I>>,
    {
        let mut comp = crate::sequence::comparison::Comparison::new(vec![self]);
        comp.add(anothers.into_iter());
        comp
    }
}

impl<I> crate::traits::Preexplorable for Sequence<I>
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

        // Create the data structure for gnuplot

        let mut raw_data = String::new();

        for (counter, value) in self.data.clone().into_iter().enumerate() {
            raw_data.push_str(&format!("{}\t{}\n", counter, value));
        }

        raw_data
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

        let gnuplot_file = &format!("{}\\{}", PLOT_DIR, format!("{}.gnu", serie));
        std::process::Command::new("gnuplot")
            .arg(gnuplot_file)
            .spawn()?;
        Ok(self)
    }

    /// Write simple gnuplot script for this type of data.
    ///
    fn plot_script<S: Display>(&self, serie: S) -> String {

        let mut gnuplot_script = self.base_plot_script();

        let dashtype = match self.get_dashtype() {
            Some(dashtype) => dashtype,
            None => 1,
        };
        gnuplot_script += &format!(
            "plot \"{}/{}.txt\" with {} dashtype {} \n",
            DATA_DIR_GNUPLOT,
            serie,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_style() {
        let data = 0..2;
        let mut seq = Sequence::new(data);
        seq.style("points");

        assert_eq!(
            &crate::configuration::plot::style::Style::Points,
            seq.get_style()
        );
    }
}

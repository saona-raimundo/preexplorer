use crate::errors::SavingError;
pub use comparison::Comparison;

/// Compare various ``Sequence`` types together.
pub mod comparison;
/// Sequence with values with n-dimensions.
pub mod ndsequence;

pub use crate::traits::Preexplorable;

// Trait bounds
use core::fmt::Display;

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
    I: ExactSizeIterator + Clone,
    I::Item: Display,
{
    pub(crate) data: I,
    pub(crate) config: crate::configuration::Configuration,
}

impl<I> Sequence<I>
where
    I: ExactSizeIterator + Clone,
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
    fn save<S: Display>(&self, serie: S) -> Result<&Self, SavingError> {
        let serie = &serie.to_string();
        self.write_plot_script(serie)?;

        // Files creation

        let data_dir = "data";
        std::fs::create_dir_all(data_dir)?;

        let data_name = &format!("{}.{}", serie, self.extension());
        let path = &format!("{}\\{}", data_dir, data_name);

        // Create the data structure for gnuplot

        let mut data_gnuplot = String::new();
        if self.header() {
            data_gnuplot.push_str(&format!("# {}", serie));
            match self.title() {
                Some(title) => data_gnuplot.push_str(&format!(": {}\n", title)),
                None => data_gnuplot.push_str("\n"),
            }
            data_gnuplot.push_str("# index value\n");
        }
        for (counter, value) in self.data.clone().enumerate() {
            data_gnuplot.push_str(&format!("{}\t{}\n", counter, value));
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

        let mut gnuplot_script = self.config.base_plot_script();

        gnuplot_script += &format!("plot \"data/{}.txt\" with {} \n", 
            serie,
            self.style(),
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_style() {
        let data = 0..2;
        let mut seq = Sequence::new(data);
        seq.set_style("points");

        assert_eq!(&crate::configuration::plot::style::Style::Points, seq.style());
    }
}
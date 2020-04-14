// Structs


// Traits
pub use crate::traits::{Configurable, Saveable, Plotable, Comparison};
use core::fmt::Display;


/// Compare various ``Sequence`` types together.
pub mod comparison;

pub use comparison::Sequences;




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

#[derive(Debug, PartialEq, Clone)]
pub struct Sequence<T> 
where
	T: Display,
{
    pub(crate) data: Vec<T>,
    pub(crate) config: crate::configuration::Configuration,
}

impl<T> Sequence<T>
where
    T: Display,
{
    pub fn new<I>(data: I) -> Sequence<T> 
    where
    	I: IntoIterator<Item=T>,
    {
        let data: Vec<T> = data.into_iter().collect();
        let config = crate::configuration::Configuration::default();

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
    pub fn compare_with<J>(self, others: J) -> crate::sequence::comparison::Sequences<T>
    where
        J: IntoIterator<Item = crate::sequence::Sequence<T>>,
    {
        let mut comp: Sequences<T> = self.into();
        comp.add_many(others);
        comp
    }

    pub fn to_comparison(self) -> crate::sequence::comparison::Sequences<T> {
        self.into()
    }
}

impl<T> Configurable for Sequence<T>
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

impl<T> Saveable for Sequence<T>
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
        let mut plotable_data = String::new();

        for (counter, value) in self.data.clone().into_iter().enumerate() {
            plotable_data.push_str(&format!("{}\t{}\n", counter, value));
        }

        plotable_data
    }
}

impl<T> Plotable for Sequence<T>
where
    T: Display + Clone,
{
    /// Write simple gnuplot script for this type of data.
    ///
    fn plot_script(&self) -> String {

        let mut gnuplot_script = self.opening_plot_script();

        let dashtype = match self.get_dashtype() {
            Some(dashtype) => dashtype,
            None => 1,
        };
        gnuplot_script += &format!(
            "plot {:?} with {} dashtype {} \n",
            self.get_data_path(),
            self.get_style(),
            dashtype,
        );
        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}


///////////////////////////////////////////////
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

// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;
use core::ops::Add;

pub mod comparison;

pub use comparison::Ternaries;

/// [Ternary plot] uses the two-dimensional simplex instead of the usual two dimensional axis.
/// In particular, it represents points of the form `(p_1, p_2, p_3)` such that
/// all coordinates are greater or equal to zero and sum up to one.
///
/// # Remarks
///
/// Values will be automatically normalized (per entry) for plotting, but not for saving. 
/// In particular, the data point `(1, 1, 0)` will be saved as such, but will be ploted
/// as the probability distribution `(0.5, 0.5, 0.0)`.
///
/// # Examples
///
/// Quick plot.
/// ```no_run
/// use preexplorer::prelude::*;
/// let values = [[0.5, 0.5, 0.0], [0.3, 0.3, 0.4], [1.0, 0.0, 0.0]];
/// pre::Ternary::new(values).plot("my_identifier").unwrap();
/// ```
///
/// Compare [Ternary] structs.
/// ```no_run
/// use preexplorer::prelude::*;
/// pre::Ternaries::new([
///     pre::Ternary::new([[0.5, 0.5, 0.0]]);
///     pre::Ternary::new([[0.0, 0.5, 0.5]]);
///     ])
///     .plot("my_identifier").unwrap();
/// ```
///
/// [Ternary plot]: https://en.wikipedia.org/wiki/Ternary_plot
/// [Ternary]: struct.Ternary.html

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ternary<T>
where
    T: Display + Clone,
{
    data: Vec<[T; 3]>,
    config: crate::configuration::Configuration,
}

impl<T> Ternary<T>
where
    T: Display + Clone,
{
    /// Constructs a new ``Ternary<T>``.
    ///
    /// # Examples
    ///
    /// From a complicated computation.
    /// ```
    /// use preexplorer::prelude::*;
    /// let data = (0..10).map(|i| [i * i + 1, i, 0] );
    /// let ternary = pre::Ternary::new(data);
    /// ```
    pub fn new<I, D>(data: I) -> Ternary<T>
    where
        I: IntoIterator<Item = [T; 3]>,
    {
        let data: Vec<[T; 3]> = data.into_iter().collect();
        let config = crate::configuration::Configuration::default();

        Ternary { data, config }
    }
}

impl<T> Add for Ternary<T>
where
    T: Display + Clone,
{
    type Output = crate::Ternaries<T>;

    fn add(self, other: crate::Ternary<T>) -> crate::Ternaries<T> {
        let mut cmp = self.into();
        cmp += other;
        cmp
    }
}

impl<T> Configurable for Ternary<T>
where
    T: Display + Clone,
{
    fn configuration_mut(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

impl<T> Saveable for Ternary<T>
where
    T: Display + Clone,
{
    fn plotable_data(&self) -> String {
        // Initial warning
        if self.data.is_empty() {
            eprintln!("Warning: There is no data.");
        }

        let mut plotable_data = String::new();

        for probability in self.data.clone() {
            plotable_data.push_str(&format!("{}\t{}\t{}\n", probability[0], probability[1], probability[2]));
        }

        plotable_data
    }
}

impl<T> Plotable for Ternary<T>
where
    T: Display + Clone,
{
    fn plot_script(&self) -> String {
        let mut gnuplot_script = self.opening_plot_script();

        //////////////////////////////////////////////////
        todo!("Check out internet scripts");
        todo!("Make your own script");

        // let dashtype = self.dashtype().unwrap_or(1);
        // gnuplot_script += &format!(
        //     "plot {:?} with {} dashtype {} \n",
        //     self.data_path(),
        //     self.style(),
        //     dashtype,
        // );
        //////////////////////////////////////////////////

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
        let data = [[0, 1, 0], [1, 0, 0], [0, 0, 1]];
        let mut seq = Ternary::new(data);
        seq.set_style("points").unwrap();

        assert_eq!(
            &crate::configuration::plot::style::Style::Points,
            seq.style()
        );
    }
}

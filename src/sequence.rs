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
///
/// use preexplorer::prelude::*;
///
/// let data = vec![0, 1, 2, 3, 4];
/// let plotting = pre::Sequence::new(&data)
///     .set_title("My Title")
///     .set_logx(-1.); // Default for gnuplot
/// plotting.plot(&"my_serie_name").unwrap();
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
        let data: I = data.into_iter().into();

        Sequence { data, config }
    }

    pub(crate) fn from_raw(data: I, config: crate::configuration::Configuration) -> Sequence<I> {
        Sequence { data, config }
    }

    pub fn set_title<S: Display>(mut self, title: S) -> Self {
        self.config.set_title(title.to_string());
        self
    }
    pub fn set_logx<N: Into<f64>>(mut self, logx: N) -> Self {
        self.config.set_logx(logx.into());
        self
    }
    pub fn set_logy<N: Into<f64>>(mut self, logy: N) -> Self {
        self.config.set_logy(logy.into());
        self
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
    /// use preexplorer::prelude::*;
    ///
    /// // Computing the data
    ///
    /// let data_1 = vec![0., 1., 2., 3., 4., 5.];
    /// let data_2 = vec![0., 1.4, 10., 4.];
    ///
    /// // Arrange everything in a vector
    ///
    /// let mut group_of_plottings = vec![
    ///     pre::Sequence::new(&data_1),
    ///     pre::Sequence::new(&data_2)
    /// ];
    ///
    /// // Create comparison and plot
    ///
    /// pre::sequence::Comparison::new(group_of_plottings)
    ///     .set_title("All together")
    ///     .plot(&"my_serie_name")
    ///     .unwrap();
    /// ```
    ///
    /// Compare some, keep computing, add to the comparison and then save/plot all together.
    ///
    /// ```no_run
    /// use preexplorer::prelude::*;
    ///
    /// // First Sequence
    ///
    /// let data_1 = vec![0., 1., 2., 3., 4., 5.];
    /// let plotting_1 = pre::Sequence::new(&data_1).set_title("First");
    ///
    /// // Add another data
    ///
    /// let data_2 = vec![0., 1.4, 10., 4.];
    /// let group_of_plottings = vec![
    ///     pre::Sequence::new(&data_2)
    ///         .set_title("Second")
    /// ];
    /// let mut comparison_plotting = plotting_1
    ///     .compare_with(group_of_plottings)
    ///     .set_title("More comparisons");
    ///
    /// // Keep adding more
    ///
    /// let data_3 = vec![0.1, 1.5, 7., 5.];
    /// let group_of_plottings = vec![
    ///     pre::Sequence::new(&data_3)
    ///         .set_title("Third")
    /// ];
    /// comparison_plotting.add(group_of_plottings);
    ///
    /// // Plot everything
    ///
    /// comparison_plotting.plot(&"my_serie_name").unwrap();
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
    fn save<S: Display>(self, serie: &S) -> Result<(), SavingError> {
        self.write_plot_script(serie)?;

        // Files creation

        let data_dir = "data";
        std::fs::create_dir_all(data_dir)?;

        let data_name = &format!("{}.txt", serie);
        let path = &format!("{}\\{}", data_dir, data_name);

        // Create the data structure for gnuplot

        let mut data_gnuplot = String::new();
        data_gnuplot.push_str("# index value\n");
        for (counter, value) in self.data.enumerate() {
            data_gnuplot.push_str(&format!("{}\t{}\n", counter, value));
        }

        // Write the data

        std::fs::write(path, data_gnuplot)?;

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
        gnuplot_script += "unset key\n";
        if let Some(title) = &self.config.title() {
            gnuplot_script += &format!("set title \"{}\"\n", title);
        }
        if let Some(logx) = &self.config.logx() {
            if *logx <= 0.0 {
                gnuplot_script += "set logscale x\n";
            } else {
                gnuplot_script += &format!("set logscale x {}\n", logx);
            }
        }
        if let Some(logy) = &self.config.logy() {
            if *logy <= 0.0 {
                gnuplot_script += "set logscale y\n";
            } else {
                gnuplot_script += &format!("set logscale y {}\n", logy);
            }
        }

        gnuplot_script += &format!("plot \"data/{}.txt\" with lines \n", serie);
        gnuplot_script += "pause -1\n";

        std::fs::write(&gnuplot_file, &gnuplot_script)?;

        Ok(())
    }
}

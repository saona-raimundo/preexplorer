pub use comparison::Comparison;

/// Compare vaious iterations.
pub mod comparison;

pub use crate::traits::PlotableStructure;

// Trait bounds
use core::fmt::Display;
use failure::{Fallible, ResultExt};

/// Iterator over the data to be consumed when saved or plotted. Can also be compared with other Iteration types.
/// 
/// # Examples
/// 
/// ```
/// 
/// use external_gnuplot::prelude::*;
/// 
/// let data = vec![0, 1, 2, 3, 4];
/// let plotting = Iteration::new(data.iter())
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
pub struct Iteration<I>
where
    I: Iterator + Clone,
    I::Item: Display,
{
    pub(crate) data: I,
    pub(crate) options: IterationOptions,
}

impl<I> Iteration<I>
where
    I: Iterator + Clone,
    I::Item: Display,
{
    pub fn new(data: I) -> Iteration<I> {
        let options = IterationOptions::default();

        Iteration { data, options }
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



    /// Compare various ``Iteration`` types together. 
    /// 
    /// You can either put all together in a vector, or add them to a ``Comparison``
    /// 
    /// # Remarks
    ///
    /// Titles of ``Iteration`` types involved in a ``Comparison`` are presented as legend. 
    /// 
    /// # Examples
    /// 
    /// Compare many ``Iteration`` types by gathering all first. 
    /// ```
    /// use external_gnuplot::prelude::*;
    /// 
    /// // Computing the data
    ///
    /// let data_1 = vec![0., 1., 2., 3., 4., 5.];
    /// let data_2 = vec![0., 1.4, 10., 4.];
    ///
    /// // Arrange everything in a vector
    ///
    /// let mut group_of_plottings = vec![];
    /// group_of_plottings.push(external_gnuplot::Iteration::new(data_1.iter()));
    /// group_of_plottings.push(external_gnuplot::Iteration::new(data_2.iter()));
    ///
    /// // Create comparison and plot
    ///
    /// external_gnuplot::iteration::Comparison::new(&mut group_of_plottings)
    ///     .set_title("All together")
    ///     .plot(&"my_serie_name")
    ///     .unwrap();
    /// ```
    /// 
    /// Compare some, keep computing, add to the comparison and then save/plot all together. 
    /// 
    /// ```
    /// use external_gnuplot::prelude::*;
    /// 
    /// // First iteration
    ///
    /// let data_1 = vec![0., 1., 2., 3., 4., 5.];
    /// let plotting_1 = external_gnuplot::Iteration::new(data_1.iter()).set_title("First");
    ///
    /// // Add another data
    ///
    /// let data_2 = vec![0., 1.4, 10., 4.];
    /// let mut group_of_plottings = vec![];
    /// group_of_plottings.push(external_gnuplot::Iteration::new(data_2.iter()).set_title("Second"));
    /// let mut comparison_plotting = plotting_1
    ///     .compare(&mut group_of_plottings)
    ///     .set_title("More comparisons");
    ///
    /// // Keep adding more
    ///
    /// let data_3 = vec![0.1, 1.5, 7., 5.];
    /// let mut group_of_plottings = vec![];
    /// group_of_plottings.push(external_gnuplot::Iteration::new(data_3.iter()).set_title("Third"));
    /// comparison_plotting.add(&group_of_plottings);
    ///
    /// // Plot everything
    ///
    /// comparison_plotting.plot(&2).unwrap();
    /// ```
    /// 
    pub fn compare(
        self,
        anothers: &mut std::vec::Vec<crate::iteration::Iteration<I>>,
    ) -> crate::iteration::comparison::Comparison<I> {
        anothers.push(self);

        crate::iteration::comparison::Comparison::new(anothers)
    }
}

impl<I> crate::traits::PlotableStructure for Iteration<I>
where
    I: Iterator + Clone,
    I::Item: Display,
{
	/// Saves the data under ``data`` directory, and writes a basic plot_script to be used after execution. 
	/// 
	/// # Remark
	/// 
	/// It is inteded for when one only wants to save the data, and not call any plotting
	/// during the Rust program execution. Posterior plotting can easily be done with the 
	/// quick template gnuplot script saved under ``plots`` directory. 
    fn save<S: Display>(mut self, serie: &S) -> Fallible<()> {

    	self.write_plot_script(serie)?;

        // Files creation

        let data_dir = "data";
        std::fs::create_dir_all(data_dir).unwrap();

        let data_name = &format!("{}.txt", serie);
        let path = &format!("{}\\{}", data_dir, data_name);

        // Create the data structure for gnuplot

        let mut data_gnuplot = String::new();
        data_gnuplot.push_str("# iteration value\n");
        let mut counter = 0;
        loop {
            match self.data.next() {
                Some(value) => {
                    data_gnuplot.push_str(&format!("{}\t{}\n", counter, value));
                    counter += 1;
                }
                None => break,
            }
        }

        // Write the data

        std::fs::write(path, data_gnuplot).context("Failed to save simulation.")?;

        Ok(())
    }

    /// Plots the data by: saving it in hard-disk, writting a plot script for gnuplot and calling it. 
    /// 
    /// # Remark
    /// 
    /// The plot will be executed asyncroniously and idependently of the Rust program. 
    /// 
    fn plot<S: Display>(self, serie: &S) -> Fallible<()> {
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
    fn write_plot_script<S: Display>(&self, serie: &S) -> Fallible<()> {
        std::fs::create_dir_all("plots").unwrap();
        let gnuplot_file = &format!("plots\\{}.gnu", serie);

        let mut gnuplot_script = String::new();
        gnuplot_script += &format!("unset key\n");
        if let Some(title) = &self.options.title {
            gnuplot_script += &format!("set title \"{}\"\n", title);
        }
        if let Some(logx) = &self.options.logx {
            if *logx == -1.0 {
                gnuplot_script += &format!("set logscale x\n");
            } else {
                gnuplot_script += &format!("set logscale x {}\n", logx);
            }
        }
        if let Some(logy) = &self.options.logy {
            if *logy == -1.0 {
                gnuplot_script += &format!("set logscale y\n");
            } else {
                gnuplot_script += &format!("set logscale y {}\n", logy);
            }
        }

        gnuplot_script += &format!("plot \"data/{}.txt\" using 1:2 with lines \n", serie);
        gnuplot_script += &format!("pause -1\n");

        std::fs::write(&gnuplot_file, &gnuplot_script)?;

        Ok(())
    }
}





#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub(crate) struct IterationOptions {
    title: Option<String>,
    logx: Option<f64>,
    logy: Option<f64>,
}

impl IterationOptions {
    pub(crate) fn default() -> IterationOptions {
        let title = None;
        let logx = None;
        let logy = None;

        IterationOptions { title, logx, logy }
    }

    pub(crate) fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }
    pub(crate) fn set_logx(&mut self, logx: f64) {
        self.logx = Some(logx);
    }
    pub(crate) fn set_logy(&mut self, logy: f64) {
        self.logy = Some(logy);
    }
}



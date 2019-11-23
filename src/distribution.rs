use crate::errors::SavingError;
pub use comparison::Comparison;

/// Compare various ``Distribution`` types together.
pub mod comparison;
/// Distribution with values with n-dimensions.
pub mod nddistribution;

pub use crate::traits::Preexplorable;

// Trait bounds
use core::fmt::Display;

/// Iterator over the data to be consumed when saved or plotted.
/// Can also be compared with other Distribution types.
///
/// # Examples
///
/// ```no_run
///
/// use preexplorer::prelude::*;
///
/// let values = (0..200).chain(0..50);
/// pre::Distribution::new(values)
/// 	.set_title("My Title")
/// 	.set_logx(2)
/// 	.plot(&"my_serie_name").unwrap();
/// ```
///
/// # Remarks
///
/// See ``compare`` method to compare two or more data sets.
///
#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Distribution<I>
where
    I: IntoIterator + Clone,
    I::Item: Into<f64> + Display + Copy,
{
    pub(crate) realizations: I,
    pub(crate) config: crate::configuration::Configuration,
}

impl<I> Distribution<I>
where
    I: IntoIterator + Clone,
    I::Item: Into<f64> + Display + Copy,
{
    pub fn new(realizations: I) -> Distribution<I> {
        let config = crate::configuration::Configuration::default();

        Distribution {
            realizations,
            config,
        }
    }

    pub fn set_title<S: Display>(&mut self, title: S) -> &mut Self {
        self.config.set_title(title.to_string());
        self
    }
    pub fn set_logx<N: Into<f64>>(&mut self, logx: N) -> &mut Self {
        self.config.set_logx(logx.into());
        self
    }

    /// Compare various ``Distribution`` types together.
    ///
    /// You can either put all together in a vector, or add them to a ``Comparison``
    ///
    /// # Remarks
    ///
    /// Titles of ``Distribution`` types involved in a ``Comparison`` are presented as legend.
    ///
    /// # Examples
    ///
    /// ```no_run
    ///
    /// use preexplorer::prelude::*;
    /// let values_1 = (0..200).chain(0..50).chain(0..50);
    /// let values_2 = (100..300).chain(100..220).chain(150..250);
    ///
    /// pre::Distribution::new(values_1)
    /// 	.set_title("My legend")
    /// 	.compare_with( vec![
    /// 		pre::Distribution::new(values_2),
    /// 		])
    /// 	.set_title("My title")
    /// 	.plot(&1).unwrap();
    /// ```

    pub fn compare_with<J>(self, anothers: J) -> crate::distribution::comparison::Comparison<I>
    where
        J: IntoIterator<Item = crate::distribution::Distribution<I>>,
    {
        let mut comp = crate::distribution::comparison::Comparison::new(vec![self]);
        comp.add(anothers.into_iter());
        comp
    }
}

impl<I> crate::traits::Preexplorable for Distribution<I>
where
    I: IntoIterator + Clone,
    I::Item: Into<f64> + Display + Copy,
{
    /// Saves the data under ``data`` directory, and writes a basic plot_script to be used after execution.
    ///
    /// # Remark
    ///
    /// It is inteded for when one only wants to save the data, and not call any plotting
    /// during the Rust program execution. Posterior plotting can easily be done with the
    /// quick template gnuplot script saved under ``plots`` directory.
    fn save<S: Display>(&self, serie: &S) -> Result<&Self, SavingError> {
        self.write_plot_script(serie)?;

        // Files creation

        let data_dir = "data";
        std::fs::create_dir_all(data_dir)?;

        let data_name = &format!("{}.txt", serie);
        let path = &format!("{}\\{}", data_dir, data_name);

        // Create the data structure for gnuplot

        let mut data_gnuplot = String::new();
        data_gnuplot.push_str("# value\n");
        for value in self.realizations.clone().into_iter() {
            data_gnuplot.push_str(&format!("{}\n", value));
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

        // Values for the histogram

        let n = 20;
        let (mut min, mut max, mut length): (f64, f64, usize) = (std::f64::MAX, std::f64::MIN, 0);
        for val in self.realizations.clone() {
            let val = val.into();
            if val < min {
                min = val;
            }
            if val > max {
                max = val;
            }
            length += 1;
        }

        // Gnuplot section

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

        gnuplot_script += &format!("nbins = {}.0 #number of bins\n", n);
        gnuplot_script += &format!("max = {} #max value\n", max);
        gnuplot_script += &format!("min = {} #min value\n", min);
        gnuplot_script += &format!("len = {}.0 #number of values\n", length);
        gnuplot_script += &format!("width = {} / nbins #width\n\n", (max - min).abs());
        gnuplot_script += "#function used to map a value to the intervals\n";
        gnuplot_script += "hist(x,width) = width * floor(x/width) + width / 2.0\n\n";
        gnuplot_script += &format!(
            "plot \"data/{}.txt\" using (hist($1,width)):(1.0/len) smooth frequency with steps\n",
            serie
        );
        gnuplot_script += "pause -1\n";

        std::fs::write(&gnuplot_file, &gnuplot_script)?;

        Ok(self)
    }
}

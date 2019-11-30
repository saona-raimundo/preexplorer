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
    I::Item: PartialOrd + Display + Copy,
{
    pub(crate) realizations: I,
    pub(crate) config: crate::configuration::Configuration,
}

impl<I> Distribution<I>
where
    I: IntoIterator + Clone,
    I::Item: PartialOrd + Display + Copy,
{
    pub fn new(realizations: I) -> Distribution<I> {
        let mut config = crate::configuration::Configuration::default();
        config.set_style(crate::configuration::plot::style::Style::Steps);

        Distribution {
            realizations,
            config,
        }
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
    I::Item: PartialOrd + Display + Copy,
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
            data_gnuplot.push_str("# value\n");
        }
        for value in self.realizations.clone() {
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
    /// Only works for real numbers
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
    /// # Remark
    ///
    /// Only works for real numbers.
    fn write_plot_script<S: Display>(&self, serie: S) -> Result<&Self, SavingError> {
        std::fs::create_dir_all("plots")?;
        let gnuplot_file = &format!("plots\\{}.gnu", serie);

        let mut gnuplot_script = self.config.base_plot_script();

        // Values for the histogram

        let n = 20;
        let (mut min, mut max, mut length);
        length = 0;
        
        let mut realizations = self.realizations.clone().into_iter();
        match realizations.next() {
            Some(value) => {
                min = value;
                max = value;
                length += 1;
                for val in realizations {
                    // let val = val.into();
                    if val < min { min = val; }
                    if val > max { max = val; }
                    length += 1;
                }

                // Gnuplot scrpit
        
                gnuplot_script += "# Warning: this script only works when the data are real numbers. \n\n";

                gnuplot_script += &format!("nbins = {}.0 #number of bins\n", n);
                gnuplot_script += &format!("max = {} #max value\n", max);
                gnuplot_script += &format!("min = {} #min value\n", min);
                gnuplot_script += &format!("len = {}.0 #number of values\n", length);
                gnuplot_script += &format!("width = ({} - {}) / nbins #width\n\n", max, min);
                gnuplot_script += "# function used to map a value to the intervals\n";
                gnuplot_script += "hist(x,width) = width * floor(x/width) + width / 2.0\n\n";
                gnuplot_script += &format!(
                    "plot \"data/{}.txt\" using (hist($1,width)):(1.0/len) smooth frequency with {}\n",
                    serie, 
                    self.style(),
                );
            },
            None => {
                std::io::Error::new(
                    std::io::ErrorKind::Other, 
                    "No data to plot: There are no realizations, so no script can be prepared.",
                );
            },
        }

        

        // Gnuplot section

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

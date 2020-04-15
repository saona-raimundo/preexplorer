//! Histogram type of plotting: point cloud, density or probability density function
//! (pdf) and cummulative density function (cdf).
//!
//! # Examples
//!
//! Quick plot.
//! ```no_run
//! use preexplorer::prelude::*;
//! pre::Density::new((0..10)).plot("my_identifier").unwrap();
//! ```
//!
//! Compare ``Density``s.
//! ```no_run
//! use preexplorer::prelude::*;
//! pre::Densities::new(vec![
//!     pre::Density::new((0..10)),
//!     pre::Density::new((0..10)),
//!     ])
//!     .plot("my_identifier").unwrap();
//! ```

// Traits
pub use crate::traits::{Comparison, Configurable, Plotable, Saveable};
use core::fmt::Display;

// Structs
pub use comparison::Densities;

/// Compare various ``Distribution`` types together.
pub mod comparison;

/// Akin to a histogram: point cloud, density and cummulative distribution.
#[derive(Debug, PartialEq, Clone)]
pub struct Density<T>
where
    T: PartialOrd + Display + Clone,
{
    realizations: Vec<T>,
    config: crate::configuration::Configuration,
}

impl<T> Density<T>
where
    T: PartialOrd + Display + Clone,
{
    /// Create a new ``Density``.
    ///
    /// # Examples
    ///
    /// From a complicated computation.
    /// ```no_run
    /// use preexplorer::prelude::*;
    /// use rand_distr::Exp1;
    /// use rand::prelude::*;
    /// let simulation_results: Vec<f64> = (0..100).map(|_| thread_rng().sample(Exp1)).collect();
    /// pre::Density::new(simulation_results)
    ///     .title("Empirical Exponential 1")
    ///     .plot("my_identifier")
    ///     .unwrap();
    /// ```
    pub fn new<I>(realizations: I) -> Density<T>
    where
        I: IntoIterator<Item = T>,
    {
        let realizations: Vec<T> = realizations.into_iter().collect();
        let mut config = crate::configuration::Configuration::default();
        config.style(crate::configuration::plot::style::Style::Histeps);
        config.custom("cdf", "true");
        config.custom("pdf", "true");
        config.custom("cloud", "true");

        Density {
            realizations,
            config,
        }
    }

    /// Convert to ``Densities`` quickly.
    pub fn to_comparison(&self) -> crate::density::comparison::Densities<T> {
        self.clone().into()
    }

    /// Compare your ``Density``s with various ``Density``s.
    ///
    /// # Remarks
    ///
    /// Titles of ``Density``s involved in a ``Densities`` are presented as legends.
    ///
    /// # Examples
    ///
    /// Compare many ``Density``s by gathering all first (in some ``IntoIterator``).
    /// ```no_run
    /// use preexplorer::prelude::*;
    /// let first_den = pre::Density::new((0..10)).title("legend").to_owned();
    /// let many_dens = (0..5).map(|_| pre::Density::new((0..10)));
    /// let mut densities = first_den.compare_with(many_dens);
    /// densities.title("Main title");
    /// ```
    pub fn compare_with<J>(self, others: J) -> crate::density::comparison::Densities<T>
    where
        J: IntoIterator<Item = crate::density::Density<T>>,
    {
        let mut comp: Densities<T> = self.into();
        comp.add_many(others);
        comp
    }

    /// Controls the plotting of the cummulative density function (cdf). 
    /// If true, it will appear in the plotting, otherwise it will not.
    /// 
    /// # Default
    /// 
    /// The default value is true. 
    /// ```
    /// # use preexplorer::prelude::*;
    /// let mut den = pre::Density::new((0..10));
    /// assert_eq!(den.get_cdf(), true);
    /// den.cdf(false);
    /// assert_eq!(den.get_cdf(), false);
    /// ```
    pub fn cdf(&mut self, cdf: bool) -> &mut Self {
    	self.configuration().custom("cdf", cdf.to_string());
    	self
    }

    /// Controls the plotting of the probability density function (pdf). 
    /// If true, it will appear in the plotting, otherwise it will not.
    /// 
    /// # Default
    /// 
    /// The default value is true. 
    /// ```
    /// # use preexplorer::prelude::*;
    /// let mut den = pre::Density::new((0..10));
    /// assert_eq!(den.get_pdf(), true);
    /// den.pdf(false);
    /// assert_eq!(den.get_pdf(), false);
    /// ```
    pub fn pdf(&mut self, pdf: bool) -> &mut Self {
    	self.configuration().custom("pdf", pdf.to_string());
    	self
    }

    /// Controls the plotting of the point cloud. 
    /// If true, it will appear in the plotting, otherwise it will not.
    /// 
    /// # Default
    /// 
    /// The default value is true. 
    /// ```
    /// # use preexplorer::prelude::*;
    /// let mut den = pre::Density::new((0..10));
    /// assert_eq!(den.get_cloud(), true);
    /// den.cloud(false);
    /// assert_eq!(den.get_cloud(), false);
    /// ```
    pub fn cloud(&mut self, cloud: bool) -> &mut Self {
    	self.configuration().custom("cloud", cloud.to_string());
    	self
    }

    pub fn get_cloud(&self) -> bool {
    	match self.configuration_as_ref().get_custom("cloud") {
    		Some(cloud) => std::str::FromStr::from_str(cloud).unwrap(),
    		None => unreachable!(),
    	}
    }

    pub fn get_pdf(&self) -> bool {
    	match self.configuration_as_ref().get_custom("pdf") {
    		Some(pdf) => std::str::FromStr::from_str(pdf).unwrap(),
    		None => unreachable!(),
    	}
    }
    pub fn get_cdf(&self) -> bool {
    	match self.configuration_as_ref().get_custom("cdf") {
    		Some(cdf) => std::str::FromStr::from_str(cdf).unwrap(),
    		None => unreachable!(),
    	}
    }
}

impl<T> Configurable for Density<T>
where
    T: PartialOrd + Display + Clone,
{
    fn configuration(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration_as_ref(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

impl<T> Saveable for Density<T>
where
    T: PartialOrd + Display + Clone,
{
    fn plotable_data(&self) -> String {
        let mut raw_data = String::new();
        for value in self.realizations.clone() {
            raw_data.push_str(&format!("{}\n", value));
        }
        raw_data
    }
}

impl<T> Plotable for Density<T>
where
    T: PartialOrd + Display + Clone,
{
    /// Construct a suitable plot script for the struct.
    ///
    /// # Remarks
    ///
    /// Only works for real numbers.
    fn plot_script(&self) -> String {
        let mut gnuplot_script = self.opening_plot_script();

        gnuplot_script += "set zeroaxis\n";
        // Values for the histogram

        let n = 20;
        let (mut min, mut max, mut length);
        length = 0;

        let mut realizations = self.realizations.clone().into_iter();
        match realizations.next() {
            Some(value) => {
                min = value.clone();
                max = value;
                length += 1;
                for val in realizations {
                    if val < min {
                        min = val.clone();
                    }
                    if val > max {
                        max = val;
                    }
                    length += 1;
                }

                // Gnuplot scrpit

                gnuplot_script +=
                    "# Warning: this script only works when the data are real numbers. \n\n";

                gnuplot_script += &format!("nbins = {}.0 #number of bins\n", n);
                gnuplot_script += &format!("max = {} #max value\n", max);
                gnuplot_script += &format!("min = {} #min value\n", min);
                gnuplot_script += &format!("len = {}.0 #number of values\n", length);
                gnuplot_script += &format!("width = ({} - {}) / nbins #width\n\n", max, min);
                gnuplot_script += "# function used to map a value to the intervals\n";
                gnuplot_script += "hist(x,width) = width * floor(x/width)\n\n";
                let dashtype = match self.get_dashtype() {
                    Some(dashtype) => dashtype,
                    None => 1,
                };

                gnuplot_script += "plot ";
                if self.get_cloud() {
                	gnuplot_script += &format!(
	                    "{:?} using 1:(0.25*rand(0)-.35)",
	                    self.get_data_path(),
	                );
                }
                if self.get_pdf() {
                	if self.get_cloud() {
                		gnuplot_script += ", \\\n\t ";
                	}
                	gnuplot_script += &format!(
                    	"{:?} using (hist($1,width)):(1./(width*len)) smooth frequency with {} dashtype {}",
                    	self.get_data_path(),
                    	self.get_style(),
                    	dashtype,
                    );
                }
                if self.get_cdf() {
                	if self.get_cloud() || self.get_pdf() {
                		gnuplot_script += ", \\\n\t ";
                	}
                	gnuplot_script += &format!(
                    	"{:?} using 1:(1.) smooth cnorm",
                    	self.get_data_path(),
                    );
                }
                gnuplot_script += "\n";
            }
            None => {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "No data to plot: There are no realizations, so no script can be prepared.",
                );
            }
        }

        // Gnuplot section

        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

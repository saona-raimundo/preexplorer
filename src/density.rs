// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;
use core::ops::Add;

// Structs
pub use comparison::Densities;

pub mod comparison;

/// A type to a histogram: point cloud, probability density, cummulative probability distribution and/or bins.
///
/// # Examples
///
/// Quick plot.
/// ```no_run
/// use preexplorer::prelude::*;
/// pre::Density::new((0..10)).plot("my_identifier").unwrap();
/// ```
///
/// Compare [Density] structs.
/// ```no_run
/// use preexplorer::prelude::*;
/// pre::Densities::new(vec![
///     pre::Density::new((0..10)),
///     pre::Density::new((0..10)),
///     ])
///     .plot("my_identifier").unwrap();
/// ```
/// 
/// [Density]: struct.Density.html
#[derive(Debug, PartialEq, Clone)]
pub struct Density<T>
where
    T: Display + Clone,
{
    pub(crate) realizations: Vec<T>,
    config: crate::configuration::Configuration,
}

impl<T> Density<T>
where
    T: Display + Clone,
{
    /// Constructs a new ``Density<T>``.
    ///
    /// # Examples
    ///
    /// From a simulation.
    /// ```no_run
    /// # use preexplorer::prelude::*;
    /// # use rand_distr::Exp1;
    /// # use rand::prelude::*;
    /// let simulation_results: Vec<f64> = (0..100).map(|_| thread_rng().sample(Exp1)).collect();
    /// pre::Density::new(simulation_results)
    ///     .set_title("Empirical Exponential 1")
    ///     .plot("my_identifier")
    ///     .unwrap();
    /// ```
    pub fn new<I>(realizations: I) -> Density<T>
    where
        I: IntoIterator<Item = T>,
    {
        let realizations: Vec<T> = realizations.into_iter().collect();
        let mut config = crate::configuration::Configuration::default();
        config.set_custom("cdf", "true");
        config.set_custom("pdf", "true");
        config.set_custom("cloud", "true");
        config.set_custom("bins", "true");

        Density {
            realizations,
            config,
        }
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
    /// assert_eq!(den.cdf(), true);
    /// den.set_cdf(false);
    /// assert_eq!(den.cdf(), false);
    /// ```
    pub fn set_cdf(&mut self, cdf: bool) -> &mut Self {
        self.configuration_mut().set_custom("cdf", cdf.to_string());
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
    /// assert_eq!(den.pdf(), true);
    /// den.set_pdf(false);
    /// assert_eq!(den.pdf(), false);
    /// ```
    pub fn set_pdf(&mut self, pdf: bool) -> &mut Self {
        self.configuration_mut().set_custom("pdf", pdf.to_string());
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
    /// assert_eq!(den.cloud(), true);
    /// den.set_cloud(false);
    /// assert_eq!(den.cloud(), false);
    /// ```
    pub fn set_cloud(&mut self, cloud: bool) -> &mut Self {
        self.configuration_mut()
            .set_custom("cloud", cloud.to_string());
        self
    }

    /// Controls the plotting of bins representation of the density.
    /// If true, it will appear in the plotting, otherwise it will not.
    ///
    /// # Default
    ///
    /// The default value is true.
    ///
    /// # Remarks
    ///
    /// The number of bins is controlled in gnuplot. Refer to the [gnuplot documentation],
    /// you want to search for the `bins`, under the `Data` section. 
    ///
    /// [gnuplot documentation]: http://www.gnuplot.info/documentation.html
    ///
    /// ```
    /// # use preexplorer::prelude::*;
    /// let mut den = pre::Density::new((0..10));
    /// assert_eq!(den.bins(), true);
    /// den.set_bins(false);
    /// assert_eq!(den.bins(), false);
    /// ```
    pub fn set_bins(&mut self, bins: bool) -> &mut Self {
        self.configuration_mut()
            .set_custom("bins", bins.to_string());
        self
    }

    pub fn cloud(&self) -> bool {
        match self.configuration().custom("cloud") {
            Some(cloud) => std::str::FromStr::from_str(cloud).unwrap(),
            None => unreachable!(),
        }
    }

    pub fn pdf(&self) -> bool {
        match self.configuration().custom("pdf") {
            Some(pdf) => std::str::FromStr::from_str(pdf).unwrap(),
            None => unreachable!(),
        }
    }
    pub fn cdf(&self) -> bool {
        match self.configuration().custom("cdf") {
            Some(cdf) => std::str::FromStr::from_str(cdf).unwrap(),
            None => unreachable!(),
        }
    }

    pub fn bins(&self) -> bool {
        match self.configuration().custom("bins") {
            Some(bins) => std::str::FromStr::from_str(bins).unwrap(),
            None => unreachable!(),
        }
    }
}

impl<T> Add for Density<T>
where
    T: Display + Clone,
{
    type Output = crate::Densities<T>;

    fn add(self, other: crate::Density<T>) -> crate::Densities<T> {
        let mut cmp = self.into();
        cmp += other;
        cmp
    }
}

impl<T> Configurable for Density<T>
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

impl<T> Saveable for Density<T>
where
    T: Display + Clone,
{
    fn plotable_data(&self) -> String {
        // Initial warning
        if self.realizations.is_empty() {
            eprintln!("Warning: There are no realizations.");
        }
        
        let mut raw_data = String::new();
        for value in self.realizations.clone() {
            raw_data.push_str(&format!("{}\n", value));
        }
        raw_data
    }
}

impl<T> Plotable for Density<T>
where
    T: Display + Clone,
{
    /// Construct a suitable plot script for the struct.
    ///
    /// # Remarks
    ///
    /// Only works for real numbers.
    fn plot_script(&self) -> String {

        // Gnuplot script
        let mut gnuplot_script = self.opening_plot_script();
        gnuplot_script += "set zeroaxis\n";
        gnuplot_script +=
            "# Warning: this script only works when the data are real numbers. \n\n";
        gnuplot_script += "set style fill solid 0.5\n\n";

        // Ploting cloud, pdf, cdf and/or bins
        let dashtype = self.dashtype().unwrap_or(1);

        gnuplot_script += "plot ";
        if self.cloud() {
            gnuplot_script +=
                &format!("{:?} using 1:(0.25*rand(0)-.35)", self.data_path(),);
            if self.pdf() || self.cdf() || self.bins() {
                gnuplot_script += ", \\\n\t ";
            }
        }
        if self.pdf() {
            gnuplot_script += &format!(
                "{:?} using 1:(1./{}) smooth kdensity with {} dashtype {}",
                self.data_path(),
                self.realizations.len(),
                self.style(),
                dashtype,
            );
            if self.cdf() || self.bins() {
                gnuplot_script += ", \\\n\t ";
            }
        }
        if self.cdf() {
            gnuplot_script += &format!("{:?} using 1:(1.) smooth cnorm", self.data_path(),);
            if self.bins() {
                gnuplot_script += ", \\\n\t ";
            }
        }
        if self.bins() {
            gnuplot_script += &format!(
                "{:?} using 1:(1./{}) bins with boxes",
                self.data_path(),
                self.realizations.len()
            );
        }
        gnuplot_script += "\n";

        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

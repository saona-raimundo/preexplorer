//! Comparison of indexed collections of histograms.
//!
//! # Examples
//!
//! Quick plot.
//! ```no_run
//! use preexplorer::prelude::*;
//! let many_pro_bin = (0..5).map(|_| pre::ProcessBin::new((0..10).map(|i| (i..10 + i))));
//! pre::ProcessBins::new(many_pro_bin).plot("my_identifier").unwrap();
//! ```
//!

// Structs
use crate::errors::PreexplorerError;
use crate::ProcessBin;

// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;
use core::ops::{Add, AddAssign};

/// Comparison counter part of ``ProcessBin`` struct.
///
#[derive(Debug, PartialEq)]
pub struct ProcessBins<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    data_set: Vec<ProcessBin<T, S>>,
    config: crate::configuration::Configuration,
}

impl<T, S> ProcessBins<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    pub fn new<I>(data_set: I) -> Self
    where
        I: IntoIterator<Item = ProcessBin<T, S>>,
    {
        let config = crate::configuration::Configuration::default();
        let data_set = data_set
            .into_iter()
            .collect::<Vec<ProcessBin<T, S>>>();
        ProcessBins { data_set, config }
    }
}

impl<T, S> From<ProcessBin<T, S>> for ProcessBins<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn from(process: ProcessBin<T, S>) -> Self {
        ProcessBins::new(vec![process])
    }
}

impl<T, S> Add<ProcessBin<T, S>> for ProcessBins<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    type Output = Self;

    fn add(mut self, other: ProcessBin<T, S>) -> Self {
        self += other;
        self
    }
}

impl<T, S> Add for ProcessBins<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl<T, S> AddAssign<ProcessBin<T, S>> for ProcessBins<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn add_assign(&mut self, other: ProcessBin<T, S>) {
        self.data_set.push(other);
    }
}

impl<T, S> AddAssign for ProcessBins<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn add_assign(&mut self, mut other: Self) {
        self.data_set.append(&mut other.data_set);
    }
}

impl<T, S> Configurable for ProcessBins<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn configuration_mut(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

impl<T, S> Saveable for ProcessBins<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn plotable_data(&self) -> String {
        let mut raw_data = String::new();
        for process_bin in self.data_set.iter() {
            raw_data += &process_bin.plotable_data();
            raw_data += "\n";
        }
        raw_data
    }

    fn save_with_id<U: Display>(&self, id: U) -> Result<&Self, PreexplorerError> {
        for (counter, process_bin) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            process_bin.save_with_id(&inner_id)?;
        }

        Ok(self)
    }
}

impl<T, S> Plotable for ProcessBins<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn plot_script(&self) -> String {
        let id = self.checked_id();
        let mut gnuplot_script = self.config.opening_plot_script_comparison();

        gnuplot_script += &format!("array PROCESS_LENGTHS[{}]\n", self.data_set.len());
        for counter in 0..self.data_set.len() {
            gnuplot_script += &format!("# ProcessBin number {}\n", counter);
            let inner_id = format!("{}_{}", id, counter);
            let mut inner_path = self.data_path().to_path_buf();
            if let Some(extension) = self.data_extension() {
                inner_path.set_file_name(&inner_id);
                inner_path.set_extension(extension);
            } else {
                inner_path.set_file_name(&inner_id);
            }

            let process_bin = &self.data_set[counter];
            gnuplot_script += &format!("BINWIDTH_{} = {}\n", counter, process_bin.binwidth);
            gnuplot_script += &format!("PROCESS_LENGTHS[{}] = {}\n", counter + 1, process_bin.image.len());
            gnuplot_script += &format!("array DATA_POINTS_{}[{}] = [", counter, process_bin.image.len());
            for i in 0..process_bin.image.len() - 1 {
                gnuplot_script += &format!("{}, ", process_bin.image[i].len());
            }
            gnuplot_script += &format!("{}]\n", process_bin.image[process_bin.image.len() - 1].len());
            gnuplot_script += &format!("array TIMES_{}[{}] = [", counter, process_bin.domain.len());
            for i in 0..process_bin.domain.len() - 1 {
                gnuplot_script += &format!("{}, ", process_bin.domain[i]);
            }
            gnuplot_script += &format!("{}]\n", process_bin.domain[process_bin.domain.len() - 1]);
            gnuplot_script += &format!("\
# Plotting each histogram
do for [i=0:{}] {{
    set table {:?}.'_partial_plot'.i
    WEIGTH = 1. / (DATA_POINTS_{}[i+1] * BINWIDTH_{})
    plot {:?} index i using 2:WEIGTH bins binwidth=BINWIDTH_{} with boxes # reference: http://www.bersch.net/gnuplot-doc/plot.html#commands-plot-datafile-bins 
    unset table
}}

",
                process_bin.domain.len() - 1,
                self.data_path().with_file_name(inner_id),
                counter,
                counter,
                inner_path,
                counter,
            ); 
        }

        // Collecting all domains in a 2d array in gnuplot
        gnuplot_script += "TIMES(j, i) = (";
        for counter in 0..self.data_set.len() {
            gnuplot_script += &format!(" j == {} ? TIMES_{}[i] :", counter, counter);
        }
        gnuplot_script += " 'error')\n\n";
          
        gnuplot_script += "set style fill transparent solid 0.5\n#Ploting the first histogram of each Process\n";  


        // Plot with titles
        for (counter, process_bin) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            let mut inner_path = self.data_path().to_path_buf();
            inner_path.set_file_name(&inner_id);

            let legend = match process_bin.title() {
                Some(leg) => String::from(leg),
                None => counter.to_string(),
            };

            if counter > 0 { gnuplot_script += "re"; }
            gnuplot_script += &format!("\
plot '{}'.'_partial_plot'.'0' using (TIMES({}, 1)):1:(TIMES({}, 1)):(TIMES({}, 1)+$2):3:4 with boxxyerrorbars linecolor {} title \"{}\" # using x:y:xlow:xhigh:ylow:yhigh
",
                inner_path.display(),
                counter,
                counter,
                counter,
                counter,
                legend,
            );
        }

        // Plot without titles
        let mut path = self.data_path().to_path_buf();
        path.set_file_name(&id);
        gnuplot_script += &format!("\
# Plotting the rest of the histograms in each Process
replot for [j=0:{}] for [i=1:PROCESS_LENGTHS[j+1]-1] '{}_'.j.'_partial_plot'.i using (TIMES(j, i+1)):1:(TIMES(j, i+1)):(TIMES(j, i+1)+$2):3:4 with boxxyerrorbars linecolor j notitle # using x:y:xlow:xhigh:ylow:yhigh
",
            self.data_set.len() - 1,
            path.display(),
        );
        gnuplot_script += "\n";
        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

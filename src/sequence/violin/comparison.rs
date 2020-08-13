//! Comparison of sequences of values with a given violin.
//!
//! # Examples
//!
//! Quick plot.
//! ```no_run
//! use preexplorer::prelude::*;
//! let many_seq_err = (0..5).map(|_| pre::SequenceViolin::new((0..10).map(|i| (i..10 + i))));
//! pre::SequenceViolins::new(many_seq_err).plot("my_identifier").unwrap();
//! ```
//!

// Structs
use crate::errors::PreexplorerError;
use crate::SequenceViolin;

// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;
use core::ops::{Add, AddAssign};

/// Comparison counter part of ``Sequence`` struct.
///
#[derive(Debug, PartialEq)]
pub struct SequenceViolins<T>
where
    T: Display + Clone,
{
    data_set: Vec<SequenceViolin<T>>,
    config: crate::configuration::Configuration,
}

impl<T> SequenceViolins<T>
where
    T: Display + Clone,
{
    pub fn new<I>(data_set: I) -> Self
    where
        I: IntoIterator<Item = SequenceViolin<T>>,
    {
        let config = crate::configuration::Configuration::default();
        let data_set = data_set
            .into_iter()
            .collect::<Vec<SequenceViolin<T>>>();
        SequenceViolins { data_set, config }
    }
}

impl<T> From<SequenceViolin<T>> for SequenceViolins<T>
where
    T: Display + Clone,
{
    fn from(sequence: SequenceViolin<T>) -> Self {
        SequenceViolins::new(vec![sequence])
    }
}

impl<T> Add<SequenceViolin<T>> for SequenceViolins<T>
where
    T: Display + Clone,
{
    type Output = Self;

    fn add(mut self, other: SequenceViolin<T>) -> Self {
        self += other;
        self
    }
}

impl<T> Add for SequenceViolins<T>
where
    T: Display + Clone,
{
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl<T> AddAssign<SequenceViolin<T>> for SequenceViolins<T>
where
    T: Display + Clone,
{
    fn add_assign(&mut self, other: SequenceViolin<T>) {
        self.data_set.push(other);
    }
}

impl<T> AddAssign for SequenceViolins<T>
where
    T: Display + Clone,
{
    fn add_assign(&mut self, mut other: Self) {
        self.data_set.append(&mut other.data_set);
    }
}

impl<T> Configurable for SequenceViolins<T>
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

impl<T> Saveable for SequenceViolins<T>
where
    T: Display + Clone,
{
    fn plotable_data(&self) -> String {
        let mut raw_data = String::new();
        for sequence_violin in self.data_set.iter() {
            raw_data += &sequence_violin.plotable_data();
            raw_data += "\n";
        }
        raw_data
    }

    fn save_with_id<S: Display>(&self, id: S) -> Result<&Self, PreexplorerError> {
        for (counter, sequence_violin) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            sequence_violin.save_with_id(&inner_id)?;
        }

        Ok(self)
    }
}

impl<T> Plotable for SequenceViolins<T>
where
    T: Display + Clone,
{
    fn plot_script(&self) -> String {
        let id = self.checked_id();
        let mut gnuplot_script = self.config.opening_plot_script_comparison();


        gnuplot_script += &format!("array RENORMALIZE[{}]\n", self.data_set.len());
        gnuplot_script += &format!("array DATA_POINTS[{}] = [", self.data_set.len());
        for counter in 0..self.data_set.iter().len() - 1 {
            gnuplot_script += &format!("{}, ", self.data_set[counter].data.len());
        }
        gnuplot_script += &format!("{}]\n", self.data_set[self.data_set.len() - 1].data.len());
        
        // gnuplot_script += &format!("do for [j=0:{}] {{\n", self.data_set.len());

        for (counter, sequence_violin) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            let mut inner_path = self.data_path().to_path_buf();
            if let Some(extension) = self.data_extension() {
                inner_path.set_file_name(&inner_id);
                inner_path.set_extension(extension);
            } else {
                inner_path.set_file_name(&inner_id);
            }
            
            gnuplot_script += &format!("\
# Precomputation for violin sequence number {} 
RENORMALIZE[{}] = 2
do for [i=0:{}] {{
    # Computing some values
    set table $_
    plot {:?} index i using 2:(1) smooth kdensity
    unset table
    RENORMALIZE[{}] = (RENORMALIZE[{}] < 2 * GPVAL_Y_MAX) ? 2 * GPVAL_Y_MAX : RENORMALIZE[{}]
    # Plotting a greater domain
    set table {:?}.'_partial_plot'.i
    x_min = (GPVAL_X_MIN < GPVAL_X_MIN - 5 * GPVAL_KDENSITY_BANDWIDTH)? GPVAL_X_MIN : GPVAL_X_MIN - 5 * GPVAL_KDENSITY_BANDWIDTH
    x_max = (GPVAL_X_MAX > GPVAL_X_MAX + 5 * GPVAL_KDENSITY_BANDWIDTH)? GPVAL_X_MAX : GPVAL_X_MAX + 5 * GPVAL_KDENSITY_BANDWIDTH
    set xrange [x_min:x_max]
    plot {:?} index i using 2:(1) smooth kdensity
    unset table
    # Clean the plotting
    unset xrange
    unset yrange
}}
",
                counter,
                counter + 1,
                sequence_violin.data.len() - 1,
                inner_path,
                counter + 1,
                counter + 1,
                counter + 1,
                self.data_path().with_file_name(inner_id),
                inner_path,
            );
        }

        gnuplot_script += "set style fill transparent solid 0.5\n";

        // Plot with titles
        for (counter, sequence_violin) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            let mut inner_path = self.data_path().to_path_buf();
            inner_path.set_file_name(&inner_id);


            let legend = match sequence_violin.title() {
                Some(leg) => String::from(leg),
                None => counter.to_string(),
            };

            if counter > 0 { gnuplot_script += "re"; }
            gnuplot_script += &format!("\
plot '{}'.'_partial_plot'.'0' using (0 + $2/RENORMALIZE[{}]):1 with filledcurve x=0 linecolor {} title \"{}\"
",
                inner_path.display(),
                counter + 1,
                counter,
                legend,
            );
        }

        // Plot without titles
        let mut path = self.data_path().to_path_buf();
        path.set_file_name(&id);
        gnuplot_script += &format!("\
# Right side
replot for [j=0:{}] for [i=1:DATA_POINTS[j+1]] '{}_'.j.'_partial_plot'.i using (i + $2/RENORMALIZE[j+1]):1 with filledcurve x=i linecolor j notitle
# Left side
replot for [j=0:{}] for [i=1:DATA_POINTS[j+1]] '{}_'.j.'_partial_plot'.i using (i - $2/RENORMALIZE[j+1]):1 with filledcurve x=i linecolor j notitle
",
            self.data_set.len() - 1,
            path.display(),
            self.data_set.len() - 1,
            path.display(),
        );
        gnuplot_script += "\n";
        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

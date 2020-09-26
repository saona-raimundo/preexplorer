// Structs
use crate::errors::PreexplorerError;
use crate::SequenceBin;

// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;
use core::ops::{Add, AddAssign};

/// Comparison counter part of [SequenceBin] struct.
///
/// # Examples
///
/// Quick plot.
/// ```no_run
/// use preexplorer::prelude::*;
/// let many_seq_bin = (0..5).map(|_| pre::SequenceBin::new((0..10).map(|i| i..10 + i), 0.5));
/// pre::SequenceBins::new(many_seq_bin).plot("my_identifier").unwrap();
/// ```
///
/// [SequenceBin]: struct.SequenceBin.html
#[derive(Debug, PartialEq)]
pub struct SequenceBins<T>
where
    T: Display + Clone,
{
    data_set: Vec<SequenceBin<T>>,
    config: crate::configuration::Configuration,
}

impl<T> SequenceBins<T>
where
    T: Display + Clone,
{
    pub fn new<I>(data_set: I) -> Self
    where
        I: IntoIterator<Item = SequenceBin<T>>,
    {
        let config = crate::configuration::Configuration::default();
        let data_set = data_set.into_iter().collect::<Vec<SequenceBin<T>>>();
        SequenceBins { data_set, config }
    }
}

impl<T> From<SequenceBin<T>> for SequenceBins<T>
where
    T: Display + Clone,
{
    fn from(sequence: SequenceBin<T>) -> Self {
        SequenceBins::new(vec![sequence])
    }
}

impl<T> Add<SequenceBin<T>> for SequenceBins<T>
where
    T: Display + Clone,
{
    type Output = Self;

    fn add(mut self, other: SequenceBin<T>) -> Self {
        self += other;
        self
    }
}

impl<T> Add for SequenceBins<T>
where
    T: Display + Clone,
{
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl<T> AddAssign<SequenceBin<T>> for SequenceBins<T>
where
    T: Display + Clone,
{
    fn add_assign(&mut self, other: SequenceBin<T>) {
        self.data_set.push(other);
    }
}

impl<T> AddAssign for SequenceBins<T>
where
    T: Display + Clone,
{
    fn add_assign(&mut self, mut other: Self) {
        self.data_set.append(&mut other.data_set);
    }
}

impl<T> Configurable for SequenceBins<T>
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

impl<T> Saveable for SequenceBins<T>
where
    T: Display + Clone,
{
    fn plotable_data(&self) -> String {
        let mut raw_data = String::new();
        for sequence_bin in self.data_set.iter() {
            raw_data += &sequence_bin.plotable_data();
            raw_data += "\n";
        }
        raw_data
    }

    fn save_with_id<S: Display>(&self, id: S) -> Result<&Self, PreexplorerError> {
        for (counter, sequence_bin) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            sequence_bin.save_with_id(&inner_id)?;
        }

        Ok(self)
    }
}

impl<T> Plotable for SequenceBins<T>
where
    T: Display + Clone,
{
    fn plot_script(&self) -> String {
        let id = self.checked_id();
        let mut gnuplot_script = self.config.opening_plot_script_comparison();

        gnuplot_script += &format!("array SEQUENCE_LENGTHS[{}]\n", self.data_set.len());
        for counter in 0..self.data_set.len() {
            gnuplot_script += &format!("# SequenceBin number {}\n", counter);
            let inner_id = format!("{}_{}", id, counter);
            let mut inner_path = self.data_path().to_path_buf();
            if let Some(extension) = self.data_extension() {
                inner_path.set_file_name(&inner_id);
                inner_path.set_extension(extension);
            } else {
                inner_path.set_file_name(&inner_id);
            }

            let sequence_bin = &self.data_set[counter];
            gnuplot_script += &format!("BINWIDTH_{} = {}\n", counter, sequence_bin.binwidth);
            gnuplot_script += &format!(
                "SEQUENCE_LENGTHS[{}] = {}\n",
                counter + 1,
                sequence_bin.data.len()
            );
            gnuplot_script += &format!(
                "array DATA_POINTS_{}[{}] = [",
                counter,
                sequence_bin.data.len()
            );
            for i in 0..sequence_bin.data.len() - 1 {
                gnuplot_script += &format!("{}, ", sequence_bin.data[i].len());
            }
            gnuplot_script += &format!(
                "{}]\n",
                sequence_bin.data[sequence_bin.data.len() - 1].len()
            );
            gnuplot_script += &format!("\
# Plotting each histogram
do for [i=0:{}] {{
    set table {:?}.'_partial_plot'.i
    plot {:?} index i using 2:(1. / (DATA_POINTS_{}[i+1] * {})) bins binwidth=BINWIDTH_{} with boxes # reference: http://www.bersch.net/gnuplot-doc/plot.html#commands-plot-datafile-bins 
    unset table
}}
",
                sequence_bin.data.len() - 1,
                self.data_path().with_file_name(inner_id),
                inner_path,
                counter,
                sequence_bin.binwidth,
                counter,
            );
        }

        gnuplot_script +=
            "set style fill transparent solid 0.5\n#Ploting the first histogram of each sequence\n";

        // Plot with titles
        for (counter, sequence_bin) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            let mut inner_path = self.data_path().to_path_buf();
            inner_path.set_file_name(&inner_id);

            let legend = match sequence_bin.title() {
                Some(leg) => String::from(leg),
                None => counter.to_string(),
            };

            if counter > 0 {
                gnuplot_script += "re";
            }
            gnuplot_script += &format!("\
plot '{}'.'_partial_plot'.'0' using (0):1:(0):(0+$2):3:4 with boxxyerrorbars linecolor {} title \"{}\" # using x:y:xlow:xhigh:ylow:yhigh
",
                inner_path.display(),
                counter,
                legend,
            );
        }

        // Plot without titles
        let mut path = self.data_path().to_path_buf();
        path.set_file_name(&id);
        gnuplot_script += &format!("\
# Plotting the rest of the histograms in each sequence
replot for [j=0:{}] for [i=1:SEQUENCE_LENGTHS[j+1]-1] '{}_'.j.'_partial_plot'.i using (i):1:(i):(i+$2):3:4 with boxxyerrorbars linecolor j notitle # using x:y:xlow:xhigh:ylow:yhigh
",
            self.data_set.len() - 1,
            path.display(),
        );
        gnuplot_script += "\n";
        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

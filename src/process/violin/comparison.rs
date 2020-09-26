// Structs
use crate::errors::PreexplorerError;
use crate::ProcessViolin;

// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;
use core::ops::{Add, AddAssign};

/// Comparison counter part of [ProcessViolin] struct.
///
/// # Examples
///
/// Quick plot.
/// ```no_run
/// use preexplorer::prelude::*;
/// let many_pro_vio = (0..5).map(|_| pre::ProcessViolin::new((2..12), (0..10).map(|i| (i..10 + i))));
/// pre::ProcessViolins::new(many_pro_vio).plot("my_identifier").unwrap();
/// ```
///
/// [ProcessViolin]: struct.ProcessViolin.html
#[derive(Debug, PartialEq)]
pub struct ProcessViolins<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    data_set: Vec<ProcessViolin<T, S>>,
    config: crate::configuration::Configuration,
}

impl<T, S> ProcessViolins<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    pub fn new<I>(data_set: I) -> Self
    where
        I: IntoIterator<Item = ProcessViolin<T, S>>,
    {
        let config = crate::configuration::Configuration::default();
        let data_set = data_set.into_iter().collect::<Vec<ProcessViolin<T, S>>>();
        ProcessViolins { data_set, config }
    }
}

impl<T, S> From<ProcessViolin<T, S>> for ProcessViolins<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn from(process: ProcessViolin<T, S>) -> Self {
        ProcessViolins::new(vec![process])
    }
}

impl<T, S> Add<ProcessViolin<T, S>> for ProcessViolins<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    type Output = Self;

    fn add(mut self, other: ProcessViolin<T, S>) -> Self {
        self += other;
        self
    }
}

impl<T, S> Add for ProcessViolins<T, S>
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

impl<T, S> AddAssign<ProcessViolin<T, S>> for ProcessViolins<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn add_assign(&mut self, other: ProcessViolin<T, S>) {
        self.data_set.push(other);
    }
}

impl<T, S> AddAssign for ProcessViolins<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn add_assign(&mut self, mut other: Self) {
        self.data_set.append(&mut other.data_set);
    }
}

impl<T, S> Configurable for ProcessViolins<T, S>
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

impl<T, S> Saveable for ProcessViolins<T, S>
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

impl<T, S> Plotable for ProcessViolins<T, S>
where
    T: Display + Clone,
    S: Display + Clone,
{
    fn plot_script(&self) -> String {
        let id = self.checked_id();
        let mut gnuplot_script = self.config.opening_plot_script_comparison();

        gnuplot_script += "\n";
        gnuplot_script += &format!("\narray PROCESS_LENGTHS[{}]\n", self.data_set.len());
        gnuplot_script += "# Possibility to renormalize each ProcessViolin by hand\n";
        gnuplot_script += &format!("array RENORMALIZE[{}] = [", self.data_set.len());
        for _ in 0..self.data_set.len() - 1 {
            gnuplot_script += "1, ";
        }
        gnuplot_script += "1]\n";

        for counter in 0..self.data_set.len() {
            gnuplot_script += &format!("# ProcessViolin number {}\n", counter);
            let inner_id = format!("{}_{}", id, counter);
            let mut inner_path = self.data_path().to_path_buf();
            inner_path.set_file_name(&inner_id);

            let mut inner_path_with_extension = inner_path.clone();
            if let Some(extension) = self.data_extension() {
                inner_path_with_extension.set_extension(extension);
            }

            let process_bin = &self.data_set[counter];

            gnuplot_script += &format!(
                "PROCESS_LENGTHS[{}] = {}\n",
                counter + 1,
                process_bin.domain.len()
            );

            gnuplot_script += &format!("\
# Pre-plotting each violin
do for [i=0:PROCESS_LENGTHS[{}] - 1] {{
    # Computing some values
    set table $_
    plot {:?} index i using 2:(1) smooth kdensity
    unset table
    # Plotting a greater domain
    set table '{}'.'_partial_plot'.i
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
                counter + 1,
                inner_path_with_extension,
                inner_path.display(),
                inner_path_with_extension,
            );

            // Forget extension
            if self.data_extension().is_some() {
                inner_path.set_extension("");
            }

            gnuplot_script += "# Define the times for this ProcessViolin\n";
            gnuplot_script += &format!("array TIMES_{}[{}] = [", counter, process_bin.domain.len());
            for i in 0..process_bin.domain.len() - 1 {
                gnuplot_script += &format!("{}, ", process_bin.domain[i]);
            }
            gnuplot_script += &format!("{}]\n", process_bin.domain[process_bin.domain.len() - 1]);
        }

        // Collecting all domains in a 2d array in gnuplot
        gnuplot_script += "TIMES(j, i) = (";
        for counter in 0..self.data_set.len() {
            gnuplot_script += &format!(" j == {} ? TIMES_{}[i] :", counter, counter);
        }
        gnuplot_script += " 'error')\n\n";

        gnuplot_script += "\
set style fill transparent solid 0.5

# Ploting the first violin of each Process\n
";

        // Plot with titles
        for (counter, process_bin) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            let mut inner_path = self.data_path().to_path_buf();
            inner_path.set_file_name(&inner_id);

            let legend = match process_bin.title() {
                Some(leg) => String::from(leg),
                None => counter.to_string(),
            };

            gnuplot_script += &format!("# ProcessViolin number {}\n", counter);
            if counter > 0 {
                gnuplot_script += "re";
            }
            // Left violin, with title
            gnuplot_script += &format!("\
plot '{}'.'_partial_plot'.'0' using (TIMES({}, 1) + $2/RENORMALIZE[{}]):1 with filledcurve x=TIMES({}, 1) linecolor {} title \"{}\"
",
                inner_path.display(),
                counter,
                counter + 1,
                counter,
                counter,
                legend,
            );
            // Right violin, without title
            gnuplot_script += &format!("\
replot '{}'.'_partial_plot'.'0' using (TIMES({}, 1) - $2/RENORMALIZE[{}]):1 with filledcurve x=TIMES({}, 1) linecolor {} notitle 
",
                inner_path.display(),
                counter,
                counter + 1,
                counter,
                counter,
            );
        }

        // Plot without titles
        let mut path = self.data_path().to_path_buf();
        path.set_file_name(&id);
        gnuplot_script += &format!("\
\n# Plotting the rest of the histograms in each Process
replot for [j=0:{}] for [i=1:PROCESS_LENGTHS[j+1]-1] '{}_'.j.'_partial_plot'.i using (TIMES(j, i+1) + $2/RENORMALIZE[j+1]):1 with filledcurve x=TIMES(j, i+1) linecolor j notitle
replot for [j=0:{}] for [i=1:PROCESS_LENGTHS[j+1]-1] '{}_'.j.'_partial_plot'.i using (TIMES(j, i+1) - $2/RENORMALIZE[j+1]):1 with filledcurve x=TIMES(j, i+1) linecolor j notitle
",
            self.data_set.len() - 1,
            path.display(),
            self.data_set.len() - 1,
            path.display(),
        );

        // Finish the script
        gnuplot_script += "\n";
        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

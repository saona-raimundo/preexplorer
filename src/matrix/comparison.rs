// Structs
use crate::errors::PreexplorerError;

// Traits
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;
use core::ops::{Add, AddAssign};

/// Comparison counter part of [Heatmap] struct.
///
/// # Examples
///
/// Quick plot.
/// ```no_run
/// # use itertools::iproduct;
/// use preexplorer::prelude::*;
/// let many_heatmaps = (0..5).map(|_| pre::Heatmap::new(0..10, 0..5, iproduct!(0..10, 0..5).map(|(x, y)| x + y)));
/// pre::Heatmaps::new(many_heatmaps).plot("my_identifier").unwrap();
/// ```
///
/// [Heatmap]: struct.Heatmap.html
#[derive(Debug, PartialEq)]
pub struct Heatmaps<T, S, U>
where
    T: Display + Clone,
    S: Display + Clone,
    U: Display + Clone,
{
    data_set: Vec<crate::Heatmap<T, S, U>>,
    config: crate::configuration::Configuration,
}

impl<T, S, U> Heatmaps<T, S, U>
where
    T: Display + Clone,
    S: Display + Clone,
    U: Display + Clone,
{
    pub fn new<I>(data_set: I) -> Heatmaps<T, S, U>
    where
        I: IntoIterator<Item = crate::Heatmap<T, S, U>>,
    {
        let config = crate::configuration::Configuration::default();
        let data_set = data_set
            .into_iter()
            .collect::<Vec<crate::Heatmap<T, S, U>>>();
        Heatmaps { data_set, config }
    }
}

impl<T, S, U> From<crate::Heatmap<T, S, U>> for Heatmaps<T, S, U>
where
    T: Display + Clone,
    S: Display + Clone,
    U: Display + Clone,
{
    fn from(heatmap: crate::Heatmap<T, S, U>) -> Self {
        Heatmaps::new(vec![heatmap])
    }
}

impl<T, S, U> Add<crate::Heatmap<T, S, U>> for Heatmaps<T, S, U>
where
    T: Display + Clone,
    S: Display + Clone,
    U: Display + Clone,
{
    type Output = Self;

    fn add(mut self, other: crate::Heatmap<T, S, U>) -> Self {
        self += other;
        self
    }
}

impl<T, S, U> Add for Heatmaps<T, S, U>
where
    T: Display + Clone,
    S: Display + Clone,
    U: Display + Clone,
{
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl<T, S, U> AddAssign<crate::Heatmap<T, S, U>> for Heatmaps<T, S, U>
where
    T: Display + Clone,
    S: Display + Clone,
    U: Display + Clone,
{
    fn add_assign(&mut self, other: crate::Heatmap<T, S, U>) {
        self.data_set.push(other);
    }
}

impl<T, S, U> AddAssign for Heatmaps<T, S, U>
where
    T: Display + Clone,
    S: Display + Clone,
    U: Display + Clone,
{
    fn add_assign(&mut self, mut other: Self) {
        self.data_set.append(&mut other.data_set);
    }
}

impl<T, S, U> Configurable for Heatmaps<T, S, U>
where
    T: Display + Clone,
    S: Display + Clone,
    U: Display + Clone,
{
    fn configuration_mut(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

impl<T, S, U> Plotable for Heatmaps<T, S, U>
where
    T: Display + Clone,
    S: Display + Clone,
    U: Display + Clone,
{
    fn plot_script(&self) -> String {
        let id = self.checked_id();
        let mut gnuplot_script = self.config.opening_plot_script_comparison();

        let rows = (self.data_set.len() as f64).sqrt().ceil();
        let columns = (self.data_set.len() as f64 / rows).ceil();
        let overall_title: &str = self.title().map(|s| s.as_str()).unwrap_or("");
        gnuplot_script += &format!(
            "set multiplot layout {},{} rowsfirst downwards title \"{}\"\n",
            rows, columns, overall_title
        );

        for (counter, heatmap) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            let mut inner_path = self.data_path().to_path_buf();
            if let Some(extension) = self.data_extension() {
                inner_path.set_file_name(&inner_id);
                inner_path.set_extension(extension);
            } else {
                inner_path.set_file_name(&inner_id);
            }
            let legend = match heatmap.title() {
                Some(leg) => String::from(leg),
                None => counter.to_string(),
            };

            gnuplot_script += &format!(
                "set title \"{}\"\nplot {:?} using 1:2:3 with image\n",
                legend, inner_path,
            );
        }
        gnuplot_script += "\n";
        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

impl<T, S, U> Saveable for Heatmaps<T, S, U>
where
    T: Display + Clone,
    S: Display + Clone,
    U: Display + Clone,
{
    fn plotable_data(&self) -> String {
        let mut raw_data = String::new();
        for heatmap in self.data_set.iter() {
            raw_data += &heatmap.plotable_data();
            raw_data += "\n";
        }
        raw_data
    }

    fn save_with_id<W: Display>(&self, id: W) -> Result<&Self, PreexplorerError> {
        for (counter, heatmap) in self.data_set.iter().enumerate() {
            let inner_id = format!("{}_{}", id, counter);
            heatmap.save_with_id(&inner_id)?;
        }
        Ok(self)
    }
}

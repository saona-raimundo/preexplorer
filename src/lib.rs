//! Save data and have a glance at it with quick plots.
//! Leave the detailed plotting to other interactive tools like gnuplot.
//!
//! Do you have a costly process in Rust and want to save the data for postprocessing?
//! Would you like to still have a basic glance to check it and leave fine-tunning of the plot for later?
//! This is the crate for you!
//!
//! # Philosophy
//!
//! Rust is great at computing, making the perfect plot takes times and Sequences.
//! This Sequences should be done externally, and do not need Rust computing power.
//! Therefore, once you achieve the data in Rust, save it, have a quick glance, and
//! leave a simple gnuplot-script to start the fine tunning of your perfect plot.
//!
//! # Remarks
//!
//! All data will be saved under the "data" folder in the main directory.
//! Plots (images or scripts) are saved under the "plots" directory.

/// Histograms or realizations of the same event that should be compare against each other.
pub mod distribution;
/// Process indexed by 1, 2, 3, ...
pub mod sequence;
/// Time-series, indexed by a subset of R.
pub mod process;

/// Common traits.
pub mod traits;
/// Errors wrapper from writting data
pub mod errors;

pub use sequence::Sequence;
pub use process::Process;
// pub use distribution::Distribution;

/// All you ussually need
pub mod prelude {

	pub use crate as ext;
    pub use crate::traits::PlotableStructure;
    // pub use crate::distribution::Distribution;
}

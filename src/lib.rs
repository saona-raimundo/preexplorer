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
//!
//! Recall that you will need to [install gnuplot](http://www.gnuplot.info/download.html)
//! to use the crate at its full potential.

/// Generic multi-dimensional data.
pub mod data;
/// Histograms or realizations of the same variable.
pub mod density;
/// Time-series, indexed by a subset of R.
pub mod process;
/// Process indexed by 1, 2, 3, ...
pub mod sequence;

/// Struct with config for explorable data. 
pub mod configuration;
/// Errors wrapper from writting data. 
pub mod errors;
/// Common traits. 
pub mod traits;


pub use data::*;
pub use density::*;
pub use process::*;
pub use sequence::*;
pub use traits::*;
pub use constants::*;

/// All you ussually need
pub mod prelude {
    pub use crate as pre;
    pub use crate::traits::*;
}

/// Directory paths. 
pub mod constants {
	pub const DATA_DIR: &str = "target\\preexplorer\\data";
	pub const DATA_DIR_GNUPLOT: &str = "target/preexplorer/data";
	pub const PLOT_DIR: &str = "target\\preexplorer\\plots";
}
	
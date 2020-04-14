//! Save data and have a glance at it with quick plots.
//! Leave the detailed plotting to other interactive tools like gnuplot.
//!
//! Do you have a costly process in Rust and want to save the data for postprocessing?
//! Would you like to still have a basic glance to check it and leave fine-tuning of the plot for later?
//! This is the crate for you!
//!
//! # Philosophy
//!
//! Rust is great at computing, making the perfect plot takes times and repetition.
//! This repetitive process in search of the perfect plot should be done externally,
//! and does not need Rust computing power.
//! Therefore, once you achieve the data in Rust, save it, have a quick glance, and
//! leave a simple gnuplot-script to start the fine tunning of your perfect plot.
//!
//! # Remarks
//!
//! All data will be saved under the folder "target\\preexplorer\\data" in the main directory.
//! Plot scripts are saved under the foleder "target\\preexplorer\\plots".
//!
//! Recall that you will need to [install gnuplot](http://www.gnuplot.info/download.html)
//! to use the crate at its full potential.
//!
//! # Examples
//!
//! Quickly check your results.
//! ```no-run
//! use preexplorer::prelude::*;
//! (0..100).map(|i| i * i)
//!     .preexplore()
//!     .plot("My first plot")
//!     .unwrap();
//! ```
//!
//! Check numerical simulations.
//! ```no-run
//! use preexplorer::prelude::*;
//! use rand_distr::Exp1;
//! use rand::prelude::*;
//! let simulation_results: Vec<f64> = (0..100).map(|_| thread_rng().sample(Exp1)).collect();
//! pre::Density::new(simulation_results)
//!     .title("Empirical Exponential 1")
//!     .plot("My first empirical distribution")
//!     .unwrap();
//! ```


/// Generic multi-dimensional data. Not automatically ploted.
pub mod data;
/// Histograms or realizations of the same variable. Empirical densities.
pub mod density;
/// Time-series, indexed by a subset of R.
pub mod process;
/// Process indexed by 1, 2, 3, ...
pub mod sequence;

/// Struct with all configurations for saving and ploting.
pub mod configuration;
/// Errors wrapper from writting data.
pub mod errors;
/// Traits for easy use or self implmentation.
pub mod traits;

pub use configuration::*;
pub use constants::*;
pub use data::*;
pub use density::*;
pub use errors::*;
pub use process::*;
pub use sequence::*;
pub use traits::*;

/// All you ussually need.
pub mod prelude {
    //! Easily start preexploring you results.
    //!
    //! # Examples
    //!
    //! Quickly check your results.
    //! ```no-run
    //! use preexplorer::prelude::*;
    //! (0..100).map(|i| i * i)
    //!     .preexplore()
    //!     .plot("My first plot")
    //!     .unwrap();
    //! ```
    //!
    //! Check numerical simulations.
    //! ```no-run
    //! use preexplorer::prelude::*;
    //! use rand_distr::Exp1;
    //! use rand::prelude::*;
    //! let simulation_results: Vec<f64> = (0..100).map(|_| thread_rng().sample(Exp1)).collect();
    //! pre::Density::new(simulation_results)
    //!     .title("Empirical Exponential 1")
    //!     .plot("My first empirical distribution")
    //!     .unwrap();
    //! ```
    pub use crate as pre;
    pub use crate::traits::*;
}

/// Directory paths.
pub mod constants {
    pub const DATA_DIR: [&str; 3] = [r"target", "preexplorer", "data"];
    pub const PLOT_DIR: [&str; 3] = [r"target", "preexplorer", "plots"];
}

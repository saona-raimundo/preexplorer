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
//! ```no_run
//! use preexplorer::prelude::*;
//! (0..100).map(|i| i * i)
//!     .preexplore()
//!     .set_title("My computations")
//!     .plot("my_identifier")
//!     .unwrap();
//! ```
//! <img src="https://user-images.githubusercontent.com/37874270/80872430-36391780-8cb2-11ea-9b84-7cb1d95f4f58.png" height="200px">
//!
//! Check numerical simulations.
//! ```no_run
//! use preexplorer::prelude::*;
//! use rand_distr::Exp1;
//! use rand::prelude::*;
//! let simulation_results: Vec<f64> = (0..100).map(|_| thread_rng().sample(Exp1)).collect();
//! pre::Density::new(simulation_results)
//!     .set_title("Empirical Exponential 1")
//!     .plot("my_identifier")
//!     .unwrap();
//! ```
//! <img src="https://user-images.githubusercontent.com/37874270/86490968-88a5bb80-bd69-11ea-913b-9124f33629c0.png" height="200px">
//!
//! Save some data (mostly numerical: matrices, simulation results and related errors, etc).
//! ```no_run
//! use preexplorer::prelude::*;
//! let my_data = vec![0., 1.1, 0.001, 2., 2.3, 0.01, 3., 1.7, 0.02]; // Some data
//! let dimension = 2;
//! pre::Data::new(my_data, dimension)
//!     .set_title("My title")
//!     .plot_later("my_identifier")
//!     .unwrap();
//! ```
//!
//! Plot some function in a grid.
//! ```no_run
//! use preexplorer::prelude::*;
//! use ndarray::Array;
//! let grid = Array::linspace(0., 1., 20);
//! let values = grid.iter().map(|x| x * x);
//! (grid.iter(), values).preexplore()
//!     .set_title("My title")
//!     .plot("my_identifier")
//!     .unwrap();
//! ```
//! <img src="https://user-images.githubusercontent.com/37874270/80872600-27069980-8cb3-11ea-9f3f-4a60e5c4d06a.png" height="200px">
//!
//! Check out the [gallery] for more.
//!
//! [gallery]: https://github.com/rasa200/preexplorer#gallery
//!
//! # Optional features
//!
//! - **`use-serde`** — Enables [serde](https://crates.io/crates/serde) support.

/// Struct with all configurations for saving and ploting.
mod configuration;
/// Generic multi-dimensional data. Not automatically ploted.
mod data;
/// Histograms or realizations of the same variable. Empirical densities.
mod density;
/// Errors wrapper from writting data.
pub mod errors;
/// (Squared) Matrices.
mod matrix;
/// Time-series, indexed by a subset of R.
mod process;
/// Process indexed by 1, 2, 3, ...
mod sequence;
/// 3-dimensional surface.
mod contour;
/// Traits for easy use or self implmentation.
pub mod traits;
/// All you ussually need.
pub mod prelude {
    //! Easily start preexploring you results.
    //!
    //! # Examples
    //!
    //! Quickly check your results.
    //! ```no_run
    //! use preexplorer::prelude::*;
    //! (0..100).map(|i| i * i)
    //!     .preexplore()
    //!     .plot("My first plot")
    //!     .unwrap();
    //! ```
    //!
    //! Check numerical simulations.
    //! ```no_run
    //! use preexplorer::prelude::*;
    //! use rand_distr::Exp1;
    //! use rand::prelude::*;
    //! let simulation_results: Vec<f64> = (0..100).map(|_| thread_rng().sample(Exp1)).collect();
    //! pre::Density::new(simulation_results)
    //!     .set_title("Empirical Exponential 1")
    //!     .plot("My first empirical distribution")
    //!     .unwrap();
    //! ```
    pub use crate as pre;
    pub use crate::traits::*;
}

/// Directory paths.
mod constants {
    /// Path the data directory.
    pub const DATA_DIR: [&str; 3] = [r"target", "preexplorer", "data"];
    /// Path the plot scripts directory.
    pub const PLOT_DIR: [&str; 3] = [r"target", "preexplorer", "plots"];
}

/// Overall, generic functions of the crate
mod functions {

    /// Removes generated artifacts
    ///
    /// Inspired by the [`cargo clean`] command, this methods removes all
    /// artifacts from the target directory that `preexplorer` has generated in the past.
    ///
    /// # Remarks
    ///
    /// This method is particulary useful when you:
    /// - run various versions of a script with identical ids.
    /// - want to delete all previous records as they are no longer useful.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following situations,
    /// but is not limited to just these cases:
    /// - The user lacks permissions to perform [`std::fs::metadata`] call
    /// on the relevant paths.
    /// - Either [`std::fs::remove_file`] or [`std::fs::remove_dir`] errors.
    ///
    /// [`cargo clean`]: https://doc.rust-lang.org/cargo/commands/cargo-clean.html
    /// [`std::fs::metadata`]: https://doc.rust-lang.org/std/fs/fn.metadata.html
    /// [`std::fs::remove_file`]: https://doc.rust-lang.org/std/fs/fn.remove_file.html
    /// [`std::fs::remove_dir`]: https://doc.rust-lang.org/std/fs/fn.remove_dir.html
    pub fn clean() -> Result<(), crate::errors::PreexplorerError> {
        let path = "./target/preexplorer";
        match std::fs::metadata(path) {
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Ok(()),
                _ => Err(crate::errors::PreexplorerError::Removing(
                    e,
                    String::from("Could not query the relevant directory."),
                )),
            },
            Ok(_) => std::fs::remove_dir_all(path).map_err(|e| {
                crate::errors::PreexplorerError::Removing(
                    e,
                    String::from("Could not remove the relevant directory."),
                )
            }),
        }
    }
}

pub use self::configuration::{Configuration, Style};
pub use self::constants::{DATA_DIR, PLOT_DIR};
pub use self::data::Data;
pub use self::density::{Densities, Density};
pub use self::functions::*;
pub use self::matrix::{Heatmap, Heatmaps};
pub use self::contour::{Contour};
pub use self::process::{
    Process, ProcessBin, ProcessBins, ProcessError, ProcessErrors, ProcessViolin, ProcessViolins,
    Processes,
};
pub use self::sequence::{
    Sequence, SequenceBin, SequenceBins, SequenceError, SequenceErrors, SequenceViolin,
    SequenceViolins, Sequences,
};
pub use self::traits::*;

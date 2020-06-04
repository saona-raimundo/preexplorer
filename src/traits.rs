//! # Implementing
//!
//! You should proceed in the following order.
//! 1. ``Configurable``
//! 2. ``Saveable``
//! 3. ``Plotable``
//!
//! ## Configurable
//!
//! Include ``Configuration`` as part of your struct, as a field.
//! This allows to handle all options. Then, include the following code.
//! ```
//! struct MyStruct{config: preexplorer::Configuration};
//! impl preexplorer::Configurable for MyStruct {
//!     fn configuration_mut(&mut self) -> &mut preexplorer::Configuration {
//!         &mut self.config
//!     }
//!     fn configuration(&self) -> &preexplorer::Configuration {
//!         &self.config
//!     }
//! }
//! ```
//!
//! ## Saveable
//!
//! Extract the data to plot from your struct in the form of a String.
//! Different from serializing your struct, you only want the data.
//!
//! ### Examples
//!
//! After implementing ``Configurable``.
//! ```
//! struct MyStruct {
//!     content: f64,
//!     config: preexplorer::Configuration,
//! };
//! # impl preexplorer::Configurable for MyStruct {
//! #     fn configuration_mut(&mut self) -> &mut preexplorer::Configuration {
//! #         &mut self.config
//! #     }
//! #     fn configuration(&self) -> &preexplorer::Configuration {
//! #         &self.config
//! #     }
//! # }
//! impl preexplorer::Saveable for MyStruct
//! {
//!     fn plotable_data(&self) -> String {
//!         let mut plotable_data = String::new();
//!         plotable_data.push_str(&format!("{}", self.content));
//!         plotable_data
//!     }
//! }
//! ```
//!
//! ## Plotable
//!
//! Write your own plot script to be executed in gnuplot.
//! You can base this script by helper functions from the ``Configurable`` trait.
//!
//! ### Examples
//!
//! After implementing ``Configurable`` and ``Saveable``.
//! ```
//! struct MyStruct {
//!     content: f64,
//!     config: preexplorer::Configuration,
//! };
//! # impl preexplorer::Configurable for MyStruct {
//! #     fn configuration_mut(&mut self) -> &mut preexplorer::Configuration {
//! #         &mut self.config
//! #     }
//! #     fn configuration(&self) -> &preexplorer::Configuration {
//! #         &self.config
//! #     }
//! # }
//! # impl preexplorer::Saveable for MyStruct
//! # {
//! #     fn plotable_data(&self) -> String {
//! #         let mut plotable_data = String::new();
//! #         plotable_data.push_str(&format!("{}", self.content));
//! #         plotable_data
//! #     }
//! # }
//! # use preexplorer::Configurable;
//! impl preexplorer::Plotable for MyStruct {
//!     fn plot_script(&self) -> String {
//!         // Start with a basis that takes into account configuration options.
//!         let mut gnuplot_script = self.opening_plot_script();
//!         // Retrieve your own options or simply personalize the plot command.
//!         let dashtype = match self.dashtype() {
//!             Some(dashtype) => dashtype,
//!             None => 1,
//!         };
//!         // Include the main plot command.
//!         gnuplot_script += &format!(
//!             "plot {:?} with {} dashtype {} \n",
//!             self.data_path(),
//!             self.style(),
//!             dashtype,
//!         );
//!         // End with other configuration options.
//!         gnuplot_script += &self.ending_plot_script();
//!         gnuplot_script
//!     }
//! }
//! ```

// Types
use crate::errors::PreexplorerError;
use std::ffi::OsStr;
use std::path::Path;

// Traits
use core::fmt::Display;

/// Quickly transform interators in ``Sequence``.
///
/// # Remarks
///
/// It is meant to be used as part of the ``prelude`` module.
/// If you want to convert into a ``Sequence`` your own struct,
/// prefer the ``Into<Sequence<T>>`` trait.  
pub trait Preexplore<I, T>
where
    I: IntoIterator<Item = T>,
    T: Display + Clone,
{
    /// Shortcut to convert into a ``Sequence``.
    fn preexplore(self) -> crate::sequence::Sequence<T>;
}

impl<I, T> Preexplore<I, T> for I
where
    I: IntoIterator<Item = T>,
    T: Display + Clone,
{
    fn preexplore(self) -> crate::sequence::Sequence<T> {
        crate::sequence::Sequence::new(self)
    }
}

/// Quickly transform tuples of interators in ``Process``.
///
/// # Remarks
///
/// It is meant to be used as part of the ``prelude`` module.
/// If you want to convert into a ``Sequence`` your own struct,
/// prefer the ``Into<Process<T, S>>`` trait.  
pub trait PreexploreProcess<I, T, J, S>
where
    I: IntoIterator<Item = T>,
    T: Display + Clone,
    J: IntoIterator<Item = S>,
    S: Display + Clone,
{
    /// Shortcut to convert into a ``Process``.
    fn preexplore(self) -> crate::process::Process<T, S>;
}

impl<I, T, J, S> PreexploreProcess<I, T, J, S> for (I, J)
where
    I: IntoIterator<Item = T>,
    T: Display + Clone,
    J: IntoIterator<Item = S>,
    S: Display + Clone,
{
    fn preexplore(self) -> crate::process::Process<T, S> {
        crate::process::Process::new(self.0, self.1)
    }
}

/// Allows basic saving and plotting configuration.
///
/// # Remarks
///
/// It is meant to be a black box for all configuration throught the
/// ``Configuration`` struct, which is pursposely kept hidden.
///
/// # Implementation
///
/// See ``traits`` module level documentation.
pub trait Configurable {
    /// Mutable access to ``Configuration``.
    fn configuration_mut(&mut self) -> &mut crate::configuration::Configuration;

    /// Reference access to ``Configuration``.
    fn configuration(&self) -> &crate::configuration::Configuration;

    /// Set title, which in comparisons correspond to legends.
    fn set_title<S: Display>(&mut self, title: S) -> &mut Self {
        self.configuration_mut().set_title(title.to_string());
        self
    }

    /// Set logaritmic scale in the x axis.
    ///
    /// # Remark
    ///
    /// The x axis that will be ploted should not include zero.
    fn set_logx<N: Into<f64>>(&mut self, logx: N) -> &mut Self {
        self.configuration_mut().set_logx(logx.into());
        self
    }

    /// Set logaritmic scale in the y axis.
    ///
    /// # Remark
    ///
    /// The y axis that will be ploted should not include zero.
    fn set_logy<N: Into<f64>>(&mut self, logy: N) -> &mut Self {
        self.configuration_mut().set_logy(logy.into());
        self
    }

    /// Set logaritmic scale in the x axis.
    ///
    /// # Remark
    ///
    /// The x axis that will be ploted should not include zero.
    /// This is a mirror method of ``logx``, for convinience.  
    fn set_xlog<N: Into<f64>>(&mut self, logx: N) -> &mut Self {
        self.set_logx(logx)
    }

    /// Set logaritmic scale in the y axis.
    ///
    /// # Remark
    ///
    /// The y axis that will be ploted should not include zero.
    /// This is a mirror method of ``logy``, for convinience.  
    fn set_ylog<N: Into<f64>>(&mut self, logy: N) -> &mut Self {
        self.set_logy(logy)
    }

    /// Set a label in the x axis.
    fn set_labelx<S: Display>(&mut self, labelx: S) -> &mut Self {
        self.configuration_mut().set_labelx(labelx.to_string());
        self
    }

    /// Set a label in the y axis.
    fn set_labely<S: Display>(&mut self, labely: S) -> &mut Self {
        self.configuration_mut().set_labely(labely.to_string());
        self
    }

    /// Set a label in the x axis.
    ///
    /// # Remarks
    ///
    /// This is a mirror method of ``labelx``, for convinience.
    fn set_xlabel<S: Display>(&mut self, labelx: S) -> &mut Self {
        self.set_labelx(labelx)
    }

    /// Set a label in the y axis.
    ///
    /// # Remarks
    ///
    /// This is a mirror method of ``labely``, for convinience.
    fn set_ylabel<S: Display>(&mut self, labely: S) -> &mut Self {
        self.set_labely(labely)
    }

    /// Set the range in the x axis. If left > right, then the x axis is inverted.
    fn set_rangex<S, T>(&mut self, left: S, right: T) -> &mut Self
    where
        f64: From<S>,
        f64: From<T>,
    {
        self.configuration_mut().set_rangex(left, right);
        self
    }

    /// Set the range in the y axis. If down > up, then the y axis is inverted.
    fn set_rangey<S, T>(&mut self, down: S, up: T) -> &mut Self
    where
        f64: From<S>,
        f64: From<T>,
    {
        self.configuration_mut().set_rangey(down, up);
        self
    }

    /// Set the range in the x axis. If left > right, then the x axis is inverted.
    ///
    /// # Remarks
    ///
    /// This is a mirror method of ``rangex``, for convinience.
    fn set_xrange<S, T>(&mut self, left: S, right: T) -> &mut Self
    where
        f64: From<S>,
        f64: From<T>,
    {
        self.set_rangex(left, right)
    }

    /// Set the range in the y axis. If down > up, then the y axis is inverted.
    ///
    /// # Remarks
    ///
    /// This is a mirror method of ``rangey``, for convinience.
    fn set_yrange<S, T>(&mut self, down: S, up: T) -> &mut Self
    where
        f64: From<S>,
        f64: From<T>,
    {
        self.set_rangey(down, up)
    }

    /// Set an extension for the data file.
    ///
    /// # Default
    ///
    /// The default value is ``txt``.
    /// ```
    /// # use preexplorer::prelude::*;
    /// let seq = (0..10).preexplore();
    /// assert_eq!(seq.data_extension().unwrap().to_str(), Some("txt"));
    /// ```
    fn set_data_extension<S: AsRef<OsStr>>(&mut self, extension: S) -> &mut Self {
        self.configuration_mut().set_data_extension(extension);
        self
    }

    /// Set an extension for the data file.
    ///
    /// # Default
    ///
    /// The default value is ``gnu``.
    /// ```
    /// # use preexplorer::prelude::*;
    /// let seq = (0..10).preexplore();
    /// assert_eq!(seq.plot_extension().unwrap().to_str(), Some("gnu"));
    /// ```
    fn set_plot_extension<S: AsRef<OsStr>>(&mut self, extension: S) -> &mut Self {
        self.configuration_mut().set_plot_extension(extension);
        self
    }

    /// Decide the presence of headers in the data file.
    /// If activated, then title, date and other information are
    /// included as a comment in the data file.
    ///
    /// # Default
    ///
    /// The default value is ``true``.
    /// ```
    /// # use preexplorer::prelude::*;
    /// let seq = (0..10).preexplore();
    /// assert_eq!(seq.header(), true);
    /// ```
    fn set_header(&mut self, header: bool) -> &mut Self {
        self.configuration_mut().set_header(header);
        self
    }

    /// Choose the style for the plot. Too see all options, go to ``Style`` struct.
    /// If you set a style and then compare with other structs, then
    /// in the joint plot, the style shall be mantained for those structs that had
    /// a style not setted by default.  
    ///
    /// # Default
    ///
    /// The default value is ``default``, which is read as ``lines``.
    /// ```
    /// # use preexplorer::prelude::*;
    /// let seq = (0..10).preexplore();
    /// assert_eq!(seq.style().to_string().as_str(), "lines");
    /// ```
    fn set_style<S>(&mut self, style: S) -> &mut Self
    where
        crate::configuration::plot::style::Style: From<S>,
    {
        self.configuration_mut().set_style(style);
        self
    }

    /// Choose the dashtype for the plot.
    /// Following the gnuplot standar, it has a cyclic behaviour.
    fn set_dashtype(&mut self, dashtype: usize) -> &mut Self {
        self.configuration_mut().set_dashtype(dashtype);
        self
    }

    /// Choose the date used when saving files.
    ///
    /// # Default
    ///
    /// Time upon creation.
    /// ```
    /// # use preexplorer::prelude::*;
    /// let seq = (0..10).preexplore();
    /// println!("{}", seq.date());
    /// ```
    fn set_date(&mut self, date: chrono::DateTime<chrono::Local>) -> &mut Self {
        self.configuration_mut().set_date(date);
        self
    }

    /// Set the unique id or file name with which both data and plot script will be saved.
    /// There is no default value and one must set it before plotting or saving.
    ///
    /// When only saving data, prefer ``saving_with_id`` method.
    ///
    /// # Remarks
    ///
    /// Commonly used methods like ``plot`` and ``plot_later``
    /// internally call this method for ease of use.
    ///
    /// # Examples
    ///
    /// Correctly identifying before saving.
    /// ```no_run
    /// # use preexplorer::prelude::*;
    /// let mut seq = (0..10).preexplore();
    /// seq.set_id("my_id").save().unwrap();
    /// ```
    ///
    /// Incorrectly identifying before saving. This panics.  
    /// ```should_panic, no_run
    /// # use preexplorer::prelude::*;
    /// (0..10).preexplore().save().unwrap();
    /// ```
    fn set_id<S: Display>(&mut self, id: S) -> &mut Self {
        self.configuration_mut().set_id(id.to_string());
        self
    }

    /// Include custom configuration fields into the ``Configuration`` struct.
    /// This is intended for all new configurations you want, specially if
    /// you are implementing the traits.
    ///
    /// # Examples
    ///
    /// Setting a subtitle option.
    /// ```
    /// # use preexplorer::prelude::*;
    /// let mut seq = (0..10).preexplore();
    /// seq.set_custom("subtitle", "My subtitle");
    /// assert_eq!(seq.custom("subtitle").unwrap().as_str(), "My subtitle");
    /// ```
    fn set_custom<S: Display, T: Display>(&mut self, key: S, value: T) -> &mut Self {
        self.configuration_mut()
            .set_custom(key.to_string(), value.to_string());
        self
    }

    /// Control tics in the x axis. Passing ``""`` shows no tics.
    /// See gnuplot documentation for a correct format.
    ///
    /// # Examples
    ///
    /// Showing only from 0 to 10, at a step of 0.35.
    /// ```
    /// # use preexplorer::prelude::*;
    /// let mut seq = (0..10).preexplore();
    /// seq.set_ticsx("0, 1.35, 10");
    /// assert_eq!(seq.ticsx().unwrap().as_str(), "0, 1.35, 10");
    /// ```
    ///
    /// Showing no tics.
    /// ```
    /// # use preexplorer::prelude::*;
    /// let mut seq = (0..10).preexplore();
    /// seq.set_ticsx("");
    /// assert_eq!(seq.ticsx().unwrap().as_str(), "");
    /// ```
    fn set_ticsx<T, S>(&mut self, ticsx: T) -> &mut Self
    where
        T: Into<Option<S>>,
        S: Display,
    {
        self.configuration_mut().set_ticsx(ticsx);
        self
    }

    /// Control tics in the y axis. Passing ``""`` shows no tics.
    /// See gnuplot documentation for a correct format.
    ///
    /// # Examples
    ///
    /// Showing only from 0 to 10, at a step of 0.35.
    /// ```
    /// # use preexplorer::prelude::*;
    /// let mut seq = (0..10).preexplore();
    /// seq.set_ticsy("0, 1.35, 10");
    /// assert_eq!(seq.ticsy().unwrap().as_str(), "0, 1.35, 10");
    /// ```
    ///
    /// Showing no tics.
    /// ```
    /// # use preexplorer::prelude::*;
    /// let mut seq = (0..10).preexplore();
    /// seq.set_ticsy("");
    /// assert_eq!(seq.ticsx().unwrap().as_str(), "");
    /// ```
    fn set_ticsy<T, S>(&mut self, ticsy: T) -> &mut Self
    where
        T: Into<Option<S>>,
        S: Display,
    {
        self.configuration_mut().set_ticsy(ticsy);
        self
    }

    /// Control tics in the x axis. See gnuplot documentation for a correct format.
    ///
    /// # Remarks
    ///
    /// This is a mirror method of ``ticsx``, for convinience.
    fn set_xtics<T, S>(&mut self, ticsx: T) -> &mut Self
    where
        T: Into<Option<S>>,
        S: Display,
    {
        self.set_ticsx(ticsx)
    }

    /// Control tics in the y axis. See gnuplot documentation for a correct format.
    ///
    /// # Remarks
    ///
    /// This is a mirror method of ``ticsy``, for convinience.
    fn set_ytics<T, S>(&mut self, ticsy: T) -> &mut Self
    where
        T: Into<Option<S>>,
        S: Display,
    {
        self.set_ticsy(ticsy)
    }

    /// Control the time for which the plot is in the screen. The unit is seconds.
    /// Any negative number means "until a key is pressed". To have no pause, pass
    /// ``0``, instead of ``None``.
    ///
    /// # Default
    ///
    /// The default value is -1.
    /// ```no_run
    /// # use preexplorer::prelude::*;
    /// let seq = (0..10).preexplore();
    /// assert_eq!(seq.pause(), Some(-1.0));
    /// ```
    ///
    /// # Examples
    ///
    /// This plot will wait two seconds before closing.
    /// ```no_run
    /// # use preexplorer::prelude::*;
    /// let mut seq = (0..10).preexplore();
    /// seq.set_pause(2);
    /// assert_eq!(seq.pause(), Some(2.0));
    /// seq.plot("two_seconds_test").unwrap();
    /// ```
    ///
    /// This plot will have no pause before closing.
    /// ```no_run
    /// # use preexplorer::prelude::*;
    /// let mut seq = (0..10).preexplore();
    /// seq.set_pause(0);
    /// assert_eq!(seq.pause(), Some(0.0));
    /// seq.plot("silent_plot").unwrap();
    /// ```
    fn set_pause<T, S>(&mut self, pause: T) -> &mut Self
    where
        T: Into<Option<S>>,
        f64: From<S>,
    {
        self.configuration_mut().set_pause(pause);
        self
    }

    //////////////////////////////////////////////////////////
    // Getting
    fn title(&self) -> Option<&String> {
        self.configuration().title()
    }
    fn logx(&self) -> Option<f64> {
        self.configuration().logx()
    }
    fn logy(&self) -> Option<f64> {
        self.configuration().logy()
    }
    fn xlog(&self) -> Option<f64> {
        self.logx()
    }
    fn ylog(&self) -> Option<f64> {
        self.logy()
    }
    fn labelx(&self) -> Option<&String> {
        self.configuration().labelx()
    }
    fn labely(&self) -> Option<&String> {
        self.configuration().labely()
    }
    fn xlabel(&self) -> Option<&String> {
        self.labelx()
    }
    fn ylabel(&self) -> Option<&String> {
        self.labely()
    }
    fn rangex(&self) -> Option<(f64, f64)> {
        self.configuration().rangex()
    }
    fn rangey(&self) -> Option<(f64, f64)> {
        self.configuration().rangey()
    }
    fn xrange(&self) -> Option<(f64, f64)> {
        self.rangex()
    }
    fn yrange(&self) -> Option<(f64, f64)> {
        self.rangey()
    }
    fn plot_extension(&self) -> Option<&OsStr> {
        self.configuration().plot_extension()
    }
    fn data_extension(&self) -> Option<&OsStr> {
        self.configuration().data_extension()
    }
    fn plot_path(&self) -> &Path {
        self.configuration().plot_path()
    }
    fn data_path(&self) -> &Path {
        self.configuration().data_path()
    }
    fn header(&self) -> bool {
        self.configuration().header()
    }
    fn style(&self) -> &crate::configuration::plot::style::Style {
        self.configuration().style()
    }
    fn dashtype(&self) -> Option<usize> {
        self.configuration().dashtype()
    }
    fn date(&self) -> &chrono::DateTime<chrono::Local> {
        self.configuration().date()
    }
    fn id(&self) -> Option<&String> {
        self.configuration().id()
    }
    fn checked_id(&self) -> &String {
        self.configuration().checked_id()
    }
    fn custom<S: Display>(&self, key: S) -> Option<&String> {
        self.configuration().custom(key.to_string())
    }
    fn ticsx(&self) -> Option<&String> {
        self.configuration().ticsx()
    }
    fn xtics(&self) -> Option<&String> {
        self.ticsx()
    }
    fn ticsy(&self) -> Option<&String> {
        self.configuration().ticsy()
    }
    fn ytics(&self) -> Option<&String> {
        self.ticsy()
    }
    fn pause(&self) -> Option<f64> {
        self.configuration().pause()
    }
}

/// Allows quick saving.
///
/// # Implementation
///
/// See ``traits`` module level documentation.
pub trait Saveable: Configurable {
    /// Extract the data from the struct.
    fn plotable_data(&self) -> String;

    /// Save the file. The directory is ``target\\preexplorer\\data\\``.
    ///
    /// # Panics
    ///
    /// If the struct has not been given an ``id``, according to the ``Configurable`` trait.
    ///
    /// # Examples
    ///
    /// Correctly identifying before saving.
    /// ```no_run
    /// # use preexplorer::prelude::*;
    /// let mut seq = (0..10).preexplore();
    /// seq.set_id("my_id").save().unwrap();
    /// ```
    ///
    /// Incorrectly identifying before saving. This panics.  
    /// ```should_panic, no_run
    /// # use preexplorer::prelude::*;
    /// (0..10).preexplore().save().unwrap();
    /// ```
    fn save(&self) -> Result<&Self, PreexplorerError> {
        let id = self.checked_id();
        self.save_with_id(id)
    }

    /// Save the file with a given ``id``.
    /// It does not change the current id to save the data.
    ///
    /// # Examples
    ///
    /// Quickly saving.
    /// ```no_run
    /// # use preexplorer::prelude::*;
    /// let mut seq = (0..10).preexplore();
    /// seq.save_with_id("quick_test").unwrap();
    /// assert_eq!(seq.id(), None);
    /// ```
    fn save_with_id<S: Display>(&self, id: S) -> Result<&Self, PreexplorerError> {
        let data_dir_path = self.data_path().parent().unwrap();
        std::fs::create_dir_all(data_dir_path)?;

        let mut path = self.data_path().to_path_buf();
        path.set_file_name(id.to_string());
        if let Some(extension) = self.data_extension() {
            path.set_extension(extension);
        };

        let mut data_gnuplot = String::new();
        if self.header() {
            if let Some(title) = self.title() {
                data_gnuplot.push_str(&format!("# {}\n", title));
            }
            if let Some(id) = self.id() {
                data_gnuplot.push_str(&format!("# {}\n", id));
            }
            data_gnuplot.push_str(&format!("# {}\n", self.date()));
        }

        data_gnuplot += &self.plotable_data();

        std::fs::write(path, data_gnuplot)?;

        Ok(self)
    }
}

/// Allows quick plotting.
///
/// # Implementation
///
/// See ``traits`` module level documentation.
pub trait Plotable: Configurable + Saveable {
    ///////////////////// Needed methods ///////////////////////////////

    /// Construct a suitable plot script for the struct.
    fn plot_script(&self) -> String;

    //////////////////////// Implemented methods /////////////////////////////

    /// Do everything except running the command of plotting.
    /// In other words:
    /// 1. Assign id.
    /// 2. Save the data.
    /// 3. Save the plot script.
    ///
    /// # Remarks
    ///
    /// Specially useful when used in ``Data`` struct. So one can write the
    /// particular plot script later.
    ///
    /// # Examples
    ///
    /// Save data and plot script for posterior analysis.  
    /// ```no_run
    /// # use preexplorer::prelude::*;
    /// # use ndarray::array;
    /// let data = array![
    ///     [1, 2, 3, 4, 5],
    ///     [2, 5, 6, 7, 8],
    ///     [3, 11, 12, 13, 14],
    /// ];
    /// let dim = 5;
    ///
    /// pre::Data::new(data.iter(), dim)
    ///     .plot_later("my_identifier")
    ///     .unwrap();
    /// ```
    fn plot_later<S: Display>(&mut self, id: S) -> Result<&mut Self, PreexplorerError> {
        self.set_id(id);
        self.write_plot_script(self.plot_script())?;
        self.save()?;

        Ok(self)
    }

    /// Main command.
    /// In other words:
    /// 1. Assign id.
    /// 2. Save the data.
    /// 3. Save the plot script.
    /// 4. Run (asynchronous) the plot script.  
    ///
    /// # Examples
    ///
    /// Quickest plot.
    /// ```no_run
    /// # use preexplorer::prelude::*;
    /// (0..10).preexplore()
    ///     .plot("my_identifier")
    ///     .unwrap();
    /// ```
    fn plot<S: Display>(&mut self, id: S) -> Result<&mut Self, PreexplorerError> {
        let id = id.to_string();
        self.set_id(id.clone());
        let gnuplot_script = self.plot_script();
        self.plot_with_script(id, gnuplot_script)?;
        Ok(self)
    }

    /// Plot with a custom script.
    /// In other words:
    /// 1. Assign id.
    /// 2. Save the data.
    /// 3. Save the custom plot script.
    /// 4. Run (asynchronous) the plot script.  
    ///
    /// # Remarks
    ///
    /// This is useful when you found a particular gnuplot script you want to plot your data
    /// with and want to do it directly from Rust. Then, you must hard-code your script in
    /// Rust (copy-paste from internet, most of the times).
    ///
    /// Note that you will have to write the full path to the data in the gnuplot format,
    /// see the example for more.
    ///
    /// # Examples
    ///
    /// Quickest plot.
    /// ```no_run
    /// # use preexplorer::prelude::*;
    /// let mut seq = (0..10).preexplore();
    /// seq.plot_with_script("my_identifier", "
    /// plot \"target/preexplorer/data/my_identifier.txt\" with linespoints linecolor 3
    /// pause 3
    /// ").unwrap();
    /// ```
    fn plot_with_script<S: Display, T: Display>(
        &mut self,
        id: S,
        script: T,
    ) -> Result<&mut Self, PreexplorerError> {
        self.set_id(id);
        self.save()?;
        self.write_plot_script(script)?;

        let gnuplot_file = self.plot_path();
        std::process::Command::new("gnuplot")
            .arg(gnuplot_file)
            .spawn()
            .map_err(|e| PreexplorerError::Plotting(e))?;
        Ok(self)
    }

    /// Write plot script given by ``plot_script`` in the machine for posterior running.
    ///
    /// # Remarks
    ///
    /// The method ``plot_later`` might be more useful.
    fn write_plot_script<S: Display>(&self, gnuplot_script: S) -> Result<&Self, PreexplorerError> {
        let path = self.plot_path().parent().unwrap();
        std::fs::create_dir_all(path)?;
        let gnuplot_file = self.plot_path();
        let gnuplot_script = gnuplot_script.to_string();

        std::fs::write(gnuplot_file, gnuplot_script)?;
        Ok(self)
    }

    /// Helper method for implementing ``Plotable``.
    fn opening_plot_script(&self) -> String {
        self.configuration().opening_plot_script()
    }

    /// Helper method for implementing ``Plotable``.
    fn ending_plot_script(&self) -> String {
        self.configuration().ending_plot_script()
    }
}

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
//!     fn configuration(&mut self) -> &mut preexplorer::Configuration {
//!         &mut self.config
//!     }
//!     fn configuration_as_ref(&self) -> &preexplorer::Configuration {
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
//! #     fn configuration(&mut self) -> &mut preexplorer::Configuration {
//! #         &mut self.config
//! #     }
//! #     fn configuration_as_ref(&self) -> &preexplorer::Configuration {
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
//! #     fn configuration(&mut self) -> &mut preexplorer::Configuration {
//! #         &mut self.config
//! #     }
//! #     fn configuration_as_ref(&self) -> &preexplorer::Configuration {
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
//!         let dashtype = match self.get_dashtype() {
//!             Some(dashtype) => dashtype,
//!             None => 1,
//!         };
//!         // Include the main plot command.
//!         gnuplot_script += &format!(
//!             "plot {:?} with {} dashtype {} \n",
//!             self.get_data_path(),
//!             self.get_style(),
//!             dashtype,
//!         );
//!         // End with other configuration options.
//!         gnuplot_script += &self.ending_plot_script();
//!         gnuplot_script
//!     }
//! }
//! ```

// Types
use crate::errors::SavingError;
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
    fn configuration(&mut self) -> &mut crate::configuration::Configuration;

    /// Reference access to ``Configuration``.
    fn configuration_as_ref(&self) -> &crate::configuration::Configuration;

    /// Set title, which in comparisons correspond to legends.
    fn title<S: Display>(&mut self, title: S) -> &mut Self {
        self.configuration().title(title.to_string());
        self
    }

    /// Set logaritmic scale in the x axis.
    ///
    /// # Remark
    ///
    /// The x axis that will be ploted should not include zero.
    fn logx<N: Into<f64>>(&mut self, logx: N) -> &mut Self {
        self.configuration().logx(logx.into());
        self
    }

    /// Set logaritmic scale in the y axis.
    ///
    /// # Remark
    ///
    /// The y axis that will be ploted should not include zero.
    fn logy<N: Into<f64>>(&mut self, logy: N) -> &mut Self {
        self.configuration().logy(logy.into());
        self
    }

    /// Set logaritmic scale in the x axis.
    ///
    /// # Remark
    ///
    /// The x axis that will be ploted should not include zero.
    /// This is a mirror method of ``logx``, for convinience.  
    fn xlog<N: Into<f64>>(&mut self, logx: N) -> &mut Self {
        self.logx(logx)
    }

    /// Set logaritmic scale in the y axis.
    ///
    /// # Remark
    ///
    /// The y axis that will be ploted should not include zero.
    /// This is a mirror method of ``logy``, for convinience.  
    fn ylog<N: Into<f64>>(&mut self, logy: N) -> &mut Self {
        self.logy(logy)
    }

    /// Set a label in the x axis.
    fn labelx<S: Display>(&mut self, labelx: S) -> &mut Self {
        self.configuration().labelx(labelx.to_string());
        self
    }

    /// Set a label in the y axis.
    fn labely<S: Display>(&mut self, labely: S) -> &mut Self {
        self.configuration().labely(labely.to_string());
        self
    }

    /// Set a label in the x axis.
    ///
    /// # Remarks
    ///
    /// This is a mirror method of ``labelx``, for convinience.
    fn xlabel<S: Display>(&mut self, labelx: S) -> &mut Self {
        self.labelx(labelx)
    }

    /// Set a label in the y axis.
    ///
    /// # Remarks
    ///
    /// This is a mirror method of ``labely``, for convinience.
    fn ylabel<S: Display>(&mut self, labely: S) -> &mut Self {
        self.labely(labely)
    }

    /// Set the range in the x axis. If left > right, then the x axis is inverted.
    fn rangex<S, T>(&mut self, left: S, right: T) -> &mut Self
    where
        f64: From<S>,
        f64: From<T>,
    {
        self.configuration().rangex(left, right);
        self
    }

    /// Set the range in the y axis. If down > up, then the y axis is inverted.
    fn rangey<S, T>(&mut self, down: S, up: T) -> &mut Self
    where
        f64: From<S>,
        f64: From<T>,
    {
        self.configuration().rangey(down, up);
        self
    }

    /// Set the range in the x axis. If left > right, then the x axis is inverted.
    ///
    /// # Remarks
    ///
    /// This is a mirror method of ``rangex``, for convinience.
    fn xrange<S, T>(&mut self, left: S, right: T) -> &mut Self
    where
        f64: From<S>,
        f64: From<T>,
    {
        self.rangex(left, right)
    }

    /// Set the range in the y axis. If down > up, then the y axis is inverted.
    ///
    /// # Remarks
    ///
    /// This is a mirror method of ``rangey``, for convinience.
    fn yrange<S, T>(&mut self, down: S, up: T) -> &mut Self
    where
        f64: From<S>,
        f64: From<T>,
    {
        self.rangey(down, up)
    }

    /// Set an extension for the data file.
    ///
    /// # Default
    ///
    /// The default value is ``txt``.
    /// ```
    /// # use preexplorer::prelude::*;
    /// let seq = (0..10).preexplore();
    /// assert_eq!(seq.get_data_extension().unwrap().to_str(), Some("txt"));
    /// ```
    fn data_extension<S: AsRef<OsStr>>(&mut self, extension: S) -> &mut Self {
        self.configuration().data_extension(extension);
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
    /// assert_eq!(seq.get_plot_extension().unwrap().to_str(), Some("gnu"));
    /// ```
    fn plot_extension<S: AsRef<OsStr>>(&mut self, extension: S) -> &mut Self {
        self.configuration().plot_extension(extension);
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
    /// assert_eq!(seq.get_header(), true);
    /// ```
    fn header(&mut self, header: bool) -> &mut Self {
        self.configuration().header(header);
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
    /// assert_eq!(seq.get_style().to_string().as_str(), "lines");
    /// ```
    fn style<S>(&mut self, style: S) -> &mut Self
    where
        crate::configuration::plot::style::Style: From<S>,
    {
        self.configuration().style(style);
        self
    }

    /// Choose the dashtype for the plot.
    /// Following the gnuplot standar, it has a cyclic behaviour.
    fn dashtype(&mut self, dashtype: usize) -> &mut Self {
        self.configuration().dashtype(dashtype);
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
    /// println!("{}", seq.get_date());
    /// ```
    fn date(&mut self, date: chrono::DateTime<chrono::Local>) -> &mut Self {
        self.configuration().date(date);
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
    /// seq.id("my_id").save().unwrap();
    /// ```
    ///
    /// Incorrectly identifying before saving. This panics.  
    /// ```should_panic, no_run
    /// # use preexplorer::prelude::*;
    /// (0..10).preexplore().save().unwrap();
    /// ```
    fn id<S: Display>(&mut self, id: S) -> &mut Self {
        self.configuration().id(id.to_string());
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
    /// seq.custom("subtitle", "My subtitle");
    /// assert_eq!(seq.get_custom("subtitle").unwrap().as_str(), "My subtitle");
    /// ```
    fn custom<S: Display, T: Display>(&mut self, key: S, value: T) -> &mut Self {
        self.configuration()
            .custom(key.to_string(), value.to_string());
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
    /// seq.ticsx("0, 1.35, 10");
    /// assert_eq!(seq.get_ticsx().unwrap().as_str(), "0, 1.35, 10");
    /// ```
    ///
    /// Showing no tics.
    /// ```
    /// # use preexplorer::prelude::*;
    /// let mut seq = (0..10).preexplore();
    /// seq.ticsx("");
    /// assert_eq!(seq.get_ticsx().unwrap().as_str(), "");
    /// ```
    fn ticsx<T, S>(&mut self, ticsx: T) -> &mut Self
    where
        T: Into<Option<S>>,
        S: Display,
    {
        self.configuration().ticsx(ticsx);
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
    /// seq.ticsy("0, 1.35, 10");
    /// assert_eq!(seq.get_ticsy().unwrap().as_str(), "0, 1.35, 10");
    /// ```
    ///
    /// Showing no tics.
    /// ```
    /// # use preexplorer::prelude::*;
    /// let mut seq = (0..10).preexplore();
    /// seq.ticsy("");
    /// assert_eq!(seq.get_ticsx().unwrap().as_str(), "");
    /// ```
    fn ticsy<T, S>(&mut self, ticsy: T) -> &mut Self
    where
        T: Into<Option<S>>,
        S: Display,
    {
        self.configuration().ticsy(ticsy);
        self
    }

    /// Control tics in the x axis. See gnuplot documentation for a correct format.
    ///
    /// # Remarks
    ///
    /// This is a mirror method of ``ticsx``, for convinience.
    fn xtics<T, S>(&mut self, ticsx: T) -> &mut Self
    where
        T: Into<Option<S>>,
        S: Display,
    {
        self.ticsx(ticsx)
    }

    /// Control tics in the y axis. See gnuplot documentation for a correct format.
    ///
    /// # Remarks
    ///
    /// This is a mirror method of ``ticsy``, for convinience.
    fn ytics<T, S>(&mut self, ticsy: T) -> &mut Self
    where
        T: Into<Option<S>>,
        S: Display,
    {
        self.ticsy(ticsy)
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
    /// assert_eq!(seq.get_pause(), Some(-1.0));
    /// ```
    ///
    /// # Examples
    ///
    /// This plot will wait two seconds before closing.
    /// ```no_run
    /// # use preexplorer::prelude::*;
    /// let mut seq = (0..10).preexplore();
    /// seq.pause(2);
    /// assert_eq!(seq.get_pause(), Some(2.0));
    /// seq.plot("two_seconds_test").unwrap();
    /// ```
    /// 
    /// This plot will have no pause before closing.
    /// ```no_run
    /// # use preexplorer::prelude::*;
    /// let mut seq = (0..10).preexplore();
    /// seq.pause(0);
    /// assert_eq!(seq.get_pause(), Some(0.0));
    /// seq.plot("silent_plot").unwrap();
    /// ```
    fn pause<T, S>(&mut self, pause: T) -> &mut Self
    where
        T: Into<Option<S>>,
        f64: From<S>,
    {
        self.configuration().pause(pause);
        self
    }

    //////////////////////////////////////////////////////////
    // Getting
    fn get_title(&self) -> Option<&String> {
        self.configuration_as_ref().get_title()
    }
    fn get_logx(&self) -> Option<f64> {
        self.configuration_as_ref().get_logx()
    }
    fn get_logy(&self) -> Option<f64> {
        self.configuration_as_ref().get_logy()
    }
    fn get_xlog(&self) -> Option<f64> {
        self.get_logx()
    }
    fn get_ylog(&self) -> Option<f64> {
        self.get_logy()
    }
    fn get_labelx(&self) -> Option<&String> {
        self.configuration_as_ref().get_labelx()
    }
    fn get_labely(&self) -> Option<&String> {
        self.configuration_as_ref().get_labely()
    }
    fn get_xlabel(&self) -> Option<&String> {
        self.get_labelx()
    }
    fn get_ylabel(&self) -> Option<&String> {
        self.get_labely()
    }
    fn get_rangex(&self) -> Option<(f64, f64)> {
        self.configuration_as_ref().get_rangex()
    }
    fn get_rangey(&self) -> Option<(f64, f64)> {
        self.configuration_as_ref().get_rangey()
    }
    fn get_xrange(&self) -> Option<(f64, f64)> {
        self.get_rangex()
    }
    fn get_yrange(&self) -> Option<(f64, f64)> {
        self.get_rangey()
    }
    fn get_plot_extension(&self) -> Option<&OsStr> {
        self.configuration_as_ref().get_plot_extension()
    }
    fn get_data_extension(&self) -> Option<&OsStr> {
        self.configuration_as_ref().get_data_extension()
    }
    fn get_plot_path(&self) -> &Path {
        self.configuration_as_ref().get_plot_path()
    }
    fn get_data_path(&self) -> &Path {
        self.configuration_as_ref().get_data_path()
    }
    fn get_header(&self) -> bool {
        self.configuration_as_ref().get_header()
    }
    fn get_style(&self) -> &crate::configuration::plot::style::Style {
        self.configuration_as_ref().get_style()
    }
    fn get_dashtype(&self) -> Option<usize> {
        self.configuration_as_ref().get_dashtype()
    }
    fn get_date(&self) -> &chrono::DateTime<chrono::Local> {
        self.configuration_as_ref().get_date()
    }
    fn get_id(&self) -> Option<&String> {
        self.configuration_as_ref().get_id()
    }
    fn get_checked_id(&self) -> &String {
        self.configuration_as_ref().get_checked_id()
    }
    fn get_custom<S: Display>(&self, key: S) -> Option<&String> {
        self.configuration_as_ref().get_custom(key.to_string())
    }
    fn get_ticsx(&self) -> Option<&String> {
        self.configuration_as_ref().get_ticsx()
    }
    fn get_xtics(&self) -> Option<&String> {
        self.get_ticsx()
    }
    fn get_ticsy(&self) -> Option<&String> {
        self.configuration_as_ref().get_ticsy()
    }
    fn get_ytics(&self) -> Option<&String> {
        self.get_ticsy()
    }
    fn get_pause(&self) -> Option<f64> {
        self.configuration_as_ref().get_pause()
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
    /// seq.id("my_id").save().unwrap();
    /// ```
    ///
    /// Incorrectly identifying before saving. This panics.  
    /// ```should_panic, no_run
    /// # use preexplorer::prelude::*;
    /// (0..10).preexplore().save().unwrap();
    /// ```
    fn save(&self) -> Result<&Self, SavingError> {
        let id = self.get_checked_id();
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
    /// assert_eq!(seq.get_id(), None);
    /// ```
    fn save_with_id<S: Display>(&self, id: S) -> Result<&Self, SavingError> {
        let data_dir_path = self.get_data_path().parent().unwrap();
        std::fs::create_dir_all(data_dir_path)?;

        let mut path = self.get_data_path().to_path_buf();
        path.set_file_name(id.to_string());
        if let Some(extension) = self.get_data_extension() {
            path.set_extension(extension);
        };

        let mut data_gnuplot = String::new();
        if self.get_header() {
            if let Some(title) = self.get_title() {
                data_gnuplot.push_str(&format!("# {}\n", title));
            }
            if let Some(id) = self.get_id() {
                data_gnuplot.push_str(&format!("# {}\n", id));
            }
            data_gnuplot.push_str(&format!("# {}\n", self.get_date()));
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
    fn plot_later(&mut self, id: &str) -> Result<&mut Self, SavingError> {
        self.id(id);
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
    fn plot(&mut self, id: &str) -> Result<&mut Self, SavingError> {
        self.id(id);
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
    fn plot_with_script<S: Display>(&mut self, id: &str, script: S) -> Result<&mut Self, SavingError> {
        self.id(id);
        self.save()?;
        self.write_plot_script(script)?;

        let gnuplot_file = self.get_plot_path();
        std::process::Command::new("gnuplot")
            .arg(gnuplot_file)
            .spawn()?;
        Ok(self)
    }

    /// Write plot script given by ``plot_script`` in the machine for posterior running.
    ///
    /// # Remarks
    ///
    /// The method ``plot_later`` might be more useful.
    fn write_plot_script<S: Display>(&self, gnuplot_script: S) -> Result<&Self, SavingError> {
        let path = self.get_plot_path().parent().unwrap();
        std::fs::create_dir_all(path)?;
        let gnuplot_file = self.get_plot_path();
        let gnuplot_script = gnuplot_script.to_string();

        std::fs::write(gnuplot_file, gnuplot_script)?;
        Ok(self)
    }

    /// Helper method for implementing ``Plotable``.
    fn opening_plot_script(&self) -> String {
        self.configuration_as_ref().opening_plot_script()
    }

    /// Helper method for implementing ``Plotable``.
    fn ending_plot_script(&self) -> String {
        self.configuration_as_ref().ending_plot_script()
    }
}

/// Basic functions for comparisons of basic structs.
pub trait Comparison<T>: From<T> {
    /// Add a basic struct to the comparison.
    fn add(&mut self, other: T) -> &mut Self;

    /// Add many basic structs to the comparison.
    fn add_many<J: IntoIterator<Item = T>>(&mut self, others: J) -> &mut Self {
        for other in others {
            self.add(other);
        }
        self
    }
}

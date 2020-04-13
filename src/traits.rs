// Types
use std::path::PathBuf;
use std::path::Path;
use std::ffi::OsStr;

use crate::errors::SavingError;

// Trait bounds
use core::fmt::Display;

// Constants
use crate::{DATA_DIR, PLOT_DIR};

pub trait Preexplore<I>
where
    I: IntoIterator + Clone,
    I::Item: Display,
{
    fn preexplore(self) -> crate::sequence::Sequence<I>;
}

impl<I> Preexplore<I> for I
where
    I: IntoIterator + Clone,
    I::Item: Display,
{
    fn preexplore(self) -> crate::sequence::Sequence<I> {
        crate::sequence::Sequence::new(self)
    }
}

pub trait PreexploreProcess<I, J>
where
    I: IntoIterator + Clone,
    I::Item: Display,
    J: IntoIterator + Clone,
    J::Item: Display,
{
    fn preexplore(self) -> crate::process::Process<I, J>;
}

impl<I, J> PreexploreProcess<I, J> for (I, J)
where
    I: IntoIterator + Clone,
    I::Item: Display,
    J: IntoIterator + Clone,
    J::Item: Display,
{
    fn preexplore(self) -> crate::process::Process<I, J> {
        crate::process::Process::new(self.0, self.1)
    }
}

pub trait Configurable {
    fn configuration(&mut self) -> &mut crate::configuration::Configuration;

    fn configuration_as_ref(&self) -> &crate::configuration::Configuration;

    fn title<S: Display>(&mut self, title: S) -> &mut Self {
        self.configuration().title(title.to_string());
        self
    }
    fn logx<N: Into<f64>>(&mut self, logx: N) -> &mut Self {
        self.configuration().logx(logx.into());
        self
    }
    fn logy<N: Into<f64>>(&mut self, logy: N) -> &mut Self {
        self.configuration().logy(logy.into());
        self
    }
    fn xlog<N: Into<f64>>(&mut self, logx: N) -> &mut Self {
        self.logx(logx)
    }
    fn ylog<N: Into<f64>>(&mut self, logy: N) -> &mut Self {
        self.logy(logy)
    }

    fn labelx<S: Display>(&mut self, labelx: S) -> &mut Self {
        self.configuration().labelx(labelx.to_string());
        self
    }
    fn labely<S: Display>(&mut self, labely: S) -> &mut Self {
        self.configuration().labely(labely.to_string());
        self
    }

    fn xlabel<S: Display>(&mut self, labelx: S) -> &mut Self {
        self.labelx(labelx)
    }
    fn ylabel<S: Display>(&mut self, labely: S) -> &mut Self {
        self.labely(labely)
    }

    fn rangex<S, T>(&mut self, left: S, right: T) -> &mut Self 
    where
        f64: From<S>,
        f64: From<T>,
    {
        self.configuration().rangex((f64::from(left), f64::from(right)));
        self
    }
    fn rangey<S, T>(&mut self, down: S, up: T) -> &mut Self 
    where
        f64: From<S>,
        f64: From<T>,
    {
        self.configuration().rangey((f64::from(down), f64::from(up)));
        self
    }
    fn xrange<S, T>(&mut self, left: S, right: T) -> &mut Self 
    where
        f64: From<S>,
        f64: From<T>,
    {
        self.rangex(left, right)
    }
    fn yrange<S, T>(&mut self, down: S, up: T) -> &mut Self 
    where
        f64: From<S>,
        f64: From<T>,
    {
        self.rangey(down, up)
    }

    fn data_extension<S: AsRef<OsStr>>(&mut self, extension: S) -> &mut Self {
        self.configuration().data_extension(extension);
        self
    }

    fn plot_extension<S: AsRef<OsStr>>(&mut self, extension: S) -> &mut Self {
        self.configuration().plot_extension(extension);
        self
    }

    fn header(&mut self, header: bool) -> &mut Self {
        self.configuration().header(header);
        self
    }
    fn style<S>(&mut self, style: S) -> &mut Self
    where
        crate::configuration::plot::style::Style: From<S>,
    {
        self.configuration()
            .style(crate::configuration::plot::style::Style::from(style));
        self
    }
    fn dashtype(&mut self, dashtype: usize) -> &mut Self {
        self.configuration().dashtype(dashtype);
        self
    }
    fn date(&mut self, date: chrono::DateTime<chrono::Local>) -> &mut Self {
        self.configuration().date(date);
        self
    }
    fn id<S: Display>(&mut self, id: S) -> &mut Self {
        self.configuration().id(id.to_string());
        self
    }

    fn custom<S: Display, T: Display>(&mut self, key: S, value: T) -> &mut Self {
        self.configuration().custom(key.to_string(), value.to_string());
        self
    }
    fn ticsx<T, S>(&mut self, ticsx: T) -> &mut Self 
    where
        T: Into<Option<S>>,
        S: Display,
    {
        let ticsx: Option<S> = ticsx.into();
        self.configuration().ticsx(ticsx.map(|t| t.to_string()));
        self
    }
    fn ticsy<T, S>(&mut self, ticsy: T) -> &mut Self 
    where
        T: Into<Option<S>>,
        S: Display,
    {
        let ticsy: Option<S> = ticsy.into();
        self.configuration().ticsy(ticsy.map(|t| t.to_string()));
        self
    }
    fn xtics<T, S>(&mut self, ticsx: T) -> &mut Self 
    where
        T: Into<Option<S>>,
        S: Display,
    {
        self.ticsx(ticsx)
    }
    fn ytics<T, S>(&mut self, ticsy: T) -> &mut Self 
    where
        T: Into<Option<S>>,
        S: Display,
    {
        self.ticsy(ticsy)
    }
    fn pause<T, S>(&mut self, pause: T) -> &mut Self 
    where
        T: Into<Option<S>>,
        f64: From<S>,
    {
        let pause: Option<S> = pause.into();
        self.configuration().pause(pause.map(|t| f64::from(t)));
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
    fn get_ticsx(&self) -> Option<&String> 
    {
        self.configuration_as_ref().get_ticsx()
    }
    fn get_xtics(&self) -> Option<&String> 
    {
        self.get_ticsx()
    }
    fn get_ticsy(&self) -> Option<&String> 
    {
        self.configuration_as_ref().get_ticsy()
    }
    fn get_ytics(&self) -> Option<&String> 
    {
        self.get_ticsy()
    }
    fn get_pause(&self) -> Option<f64> 
    {
        self.configuration_as_ref().get_pause()
    }
}

pub trait Saveable: Configurable {

    fn plotable_data(&self) -> String;

    fn save(&self) -> Result<&Self, SavingError> {
        let id = self.get_checked_id();
        self.save_with_id(id)
    }

    /// Does not change the current id to save the data. 
    fn save_with_id(&self, id: &String) -> Result<&Self, SavingError> {
        let data_dir_path: PathBuf = DATA_DIR.iter().collect();
        std::fs::create_dir_all(data_dir_path)?;

        let mut path = self.get_data_path().to_path_buf();
        path.set_file_name(id);
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

pub trait Plotable: Configurable + Saveable {
    // Needed methods

    fn plot_script(&self) -> String;

    // Implemented methods

    fn plot_later(&mut self, id: &str) -> Result<&mut Self, SavingError> {

        self.id(id);
        self.write_plot_script()?;
        self.save()?;

        Ok(self)
    }    

    fn plot(&mut self, id: &str) -> Result<&mut Self, SavingError> {

        self.plot_later(id)?;

        let gnuplot_file = self.get_plot_path();
        std::process::Command::new("gnuplot")
            .arg(gnuplot_file)
            .spawn()?;
        Ok(self)
    }

    fn write_plot_script(&self) -> Result<&Self, SavingError> {

        let path: PathBuf = PLOT_DIR.iter().collect();
        std::fs::create_dir_all(path)?;
        let gnuplot_file = self.get_plot_path();
        let gnuplot_script = self.plot_script();

        std::fs::write(gnuplot_file, gnuplot_script)?;
        Ok(self)
    }

    fn opening_plot_script(&self) -> String {
        self.configuration_as_ref().opening_plot_script()
    }

    fn ending_plot_script(&self) -> String {
        self.configuration_as_ref().ending_plot_script()
    }
}


pub trait Comparison<T>: From<T> {
    fn add(&mut self, other: T) -> &mut Self;

    fn add_many<J: IntoIterator<Item = T>>(&mut self, others: J) -> &mut Self {
        for other in others {
            self.add(other);
        }
        self
    }
}

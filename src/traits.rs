// Types
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

pub trait Preexplorable {
    // Needed methods

    fn raw_data(&self) -> String;

    fn plot(&mut self, id: &str) -> Result<&mut Self, SavingError> {

    	self.id(id);
        self.write_plot_script()?;
        self.save()?;

        let gnuplot_file = &format!("{}\\{}", PLOT_DIR, format!("{}.gnu", self.get_checked_id()));
        std::process::Command::new("gnuplot")
            .arg(gnuplot_file)
            .spawn()?;
        Ok(self)
    }

    fn plot_script(&self) -> String;

    fn configuration(&mut self) -> &mut crate::configuration::Configuration;

    fn configuration_as_ref(&self) -> &crate::configuration::Configuration;

    // Implemented methods

    fn save(&self) -> Result<&Self, SavingError> {
        let id = self.get_checked_id();
        self.save_with_id(id)
    }

    fn save_with_id(&self, id: &String) -> Result<&Self, SavingError> {

        std::fs::create_dir_all(DATA_DIR)?;

        let data_name = format!("{}.{}", id, self.get_extension());
        let path = format!("{}\\{}", DATA_DIR, data_name);


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

        data_gnuplot += &self.raw_data();

        std::fs::write(&path, data_gnuplot)?;

        Ok(self)
    }
    
    fn write_plot_script(&self) -> Result<&Self, SavingError> {

        std::fs::create_dir_all(PLOT_DIR)?;
        let gnuplot_file = format!("{}\\{}.gnu", PLOT_DIR, self.get_checked_id());
        let gnuplot_script = self.plot_script();

        std::fs::write(gnuplot_file, gnuplot_script)?;
        Ok(self)
    }

    fn base_plot_script(&self) -> String {
        self.configuration_as_ref().base_plot_script()
    }
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
    fn labelx<S: Display>(&mut self, labelx: S) -> &mut Self {
        self.configuration().labelx(labelx.to_string());
        self
    }
    fn labely<S: Display>(&mut self, labely: S) -> &mut Self {
        self.configuration().labely(labely.to_string());
        self
    }
    fn extension<S: Display>(&mut self, extension: S) -> &mut Self {
        self.configuration().extension(extension.to_string());
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


    fn get_title(&self) -> Option<&String> {
        self.configuration_as_ref().get_title()
    }
    fn get_logx(&self) -> Option<f64> {
        self.configuration_as_ref().get_logx()
    }
    fn get_logy(&self) -> Option<f64> {
        self.configuration_as_ref().get_logy()
    }
    fn get_labelx(&self) -> Option<&String> {
        self.configuration_as_ref().get_labelx()
    }
    fn get_labely(&self) -> Option<&String> {
        self.configuration_as_ref().get_labely()
    }
    fn get_extension(&self) -> &str {
        self.configuration_as_ref().get_extension()
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
}

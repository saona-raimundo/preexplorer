
/// Iterations or decrete processes indexed by natural numbers. 

pub use comparison::Comparison;

pub mod comparison;


pub use crate::traits::PlotableStructure;

// Trait bounds
use failure::{Fallible, ResultExt};
use core::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Iteration<I> 
where
	I: Iterator + Clone,
	I::Item: Display,
{
	pub(crate) data: I,
	pub(crate) options: IterationOptions,
}


impl<I> Iteration<I>
where
	I: Iterator + Clone,
	I::Item: Display,
{

	pub fn new(data: I) -> Iteration<I> {
		let options = IterationOptions::default();

		Iteration{data, options}
	}

	pub fn set_title<S: Display>(mut self, title: S) -> Self {
		self.options.set_title(title.to_string());
		self
	}
	pub fn set_logx(mut self, logx: f64) -> Self {
		self.options.set_logx(logx);
		self
	}
	pub fn set_logy(mut self, logy: f64) -> Self {
		self.options.set_logy(logy);
		self
	}

	pub fn compare(self, anothers: &mut std::vec::Vec<crate::iteration::Iteration<I>>) 
	-> crate::iteration::comparison::Comparison<I> {
		anothers.push(self);

		crate::iteration::comparison::Comparison::new(anothers)
	}
}


impl<I> crate::traits::PlotableStructure for Iteration<I>
where
	I: Iterator + Clone,
	I::Item: Display,
{	
	fn save<S: Display>(mut self, serie: &S) -> Fallible<()> {

		// Files creation

	    let data_dir = "data";
	    std::fs::create_dir_all(data_dir).unwrap();

	    let data_name = &format!("{}.txt", serie);
	    let path = &format!("{}\\{}", data_dir, data_name);

	    // Create the data structure for gnuplot

	    let mut data_gnuplot = String::new();
	    data_gnuplot.push_str("# iteration value\n");
	    let mut counter = 0;
	    loop {
	    	match self.data.next() {
	    		Some(value) => {
	    			data_gnuplot.push_str(&format!("{}\t{}\n", counter, value));
	    			counter += 1;
	    		},
	    		None => break,
	    	}
	    }

	    // Write the data

	    std::fs::write(path, data_gnuplot).context("Failed to save simulation.")?;

	    Ok(())
	}

	fn plot<S: Display>(self, serie: &S) -> Fallible<()> {

		self.write_plot_script(serie)?;
		self.save(serie)?;    	

		let gnuplot_file = format!("{}.gnu", serie);

		let gnuplot_file = &format!("plots\\{}", gnuplot_file);
		std::process::Command::new("gnuplot").arg(gnuplot_file).spawn()?;
	    Ok(())
	}

	fn write_plot_script<S: Display>(&self, serie: &S) -> Fallible<()> {
		std::fs::create_dir_all("plots").unwrap();
    	let gnuplot_file = &format!("plots\\{}.gnu", serie);

	    let mut gnuplot_script = String::new();
	    gnuplot_script += &format!("unset key\n");
	    if let Some(title) = &self.options.title {
	    	gnuplot_script += &format!("set title \"{}\"\n", title);
	    }
	    if let Some(logx) = &self.options.logx {
	    	if *logx == -1.0 {
	    		gnuplot_script += &format!("set logscale x\n");
	    	}
	    	else {
	    		gnuplot_script += &format!("set logscale x {}\n", logx);
	    	}
	    }
	    if let Some(logy) = &self.options.logy {
	    	if *logy == -1.0 {
	    		gnuplot_script += &format!("set logscale y\n");
	    	}
	    	else {
	    		gnuplot_script += &format!("set logscale y {}\n", logy);
	    	}
	    }

    	gnuplot_script += &format!("plot \"data/{}.txt\" using 1:2 with lines \n", serie);
    	gnuplot_script += &format!("pause -1\n");

    	std::fs::write(&gnuplot_file, &gnuplot_script)?;

    	Ok(())
	}

}


#[derive(Debug, PartialEq, Clone)]
pub(crate) struct IterationOptions {
	title: Option<String>,
	logx: Option<f64>,
	logy: Option<f64>,
}

impl IterationOptions {

	pub(crate) fn default() -> IterationOptions {
		let title = None;
		let logx = None;
		let logy = None;


		IterationOptions{title, logx, logy}
	}

	pub(crate) fn set_title(&mut self, title: String) {
		self.title = Some(title);
	}
	pub(crate) fn set_logx(&mut self, logx: f64) {
		self.logx = Some(logx);
	}
	pub(crate) fn set_logy(&mut self, logy: f64) {
		self.logy = Some(logy);
	}

}


#[cfg(test)]
mod tests {
	// use super::*;

	#[test]
	fn iteration_creation() {
		unimplemented!()
	}
}
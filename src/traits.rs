// Trait bounds
use core::fmt::Display;
use failure::Fallible;


pub trait PlotableStructure {
	

	fn save<S: Display>(self, serie: &S) -> Fallible<()>;

	fn plot<S: Display>(self, serie: &S) -> Fallible<()>;

	fn write_plot_script<S: Display>(&self, serie: &S) -> Fallible<()>;

}


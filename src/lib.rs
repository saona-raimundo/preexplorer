/// Easily derive results in Rust computations for after-processing with gnuplot. 

pub mod iteration;
pub mod process;
pub mod distribution;

pub mod traits;


pub use iteration::Iteration;
// pub use process::Process;
// pub use distribution::Distribution;

pub mod prelude {
	
	pub use crate::traits::PlotableStructure;
	pub use crate::iteration::Iteration;
	// pub use crate::process::Process;
	// pub use crate::distribution::Distribution;


}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

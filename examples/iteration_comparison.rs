use external_gnuplot::prelude::*;

fn main() {

	// Comparing many iterations
	
	// Computing the data
	
	let data_1 = vec![0., 1., 2., 3., 4., 5.];
	let data_2 = vec![0., 1.4, 10., 4.];
	
	// Arrange everything in a vector

	let mut group_of_plottings = vec![];
	group_of_plottings.push(external_gnuplot::Iteration::new(data_1.iter()));
	group_of_plottings.push(external_gnuplot::Iteration::new(data_2.iter()));

	// Create comparison and plot

	external_gnuplot::iteration::Comparison::new(&mut group_of_plottings)
		.set_title("All together")
		.plot(&1)
		.unwrap();


	// Comparing to iterations in an increasing manner

	// First iteration

	let data_1 = vec![0., 1., 2., 3., 4., 5.];
	let plotting_1 = external_gnuplot::Iteration::new(data_1.iter())
		.set_title("First");


	// Add another data

	let data_2 = vec![0., 1.4, 10., 4.];
	let mut group_of_plottings = vec![];
	group_of_plottings.push(external_gnuplot::Iteration::new(data_2.iter())
		.set_title("Second"));	
	let mut comparison_plotting = plotting_1.compare(&mut group_of_plottings)
		.set_title("More comparisons");

	// Keep adding more

	let data_3 = vec![0.1, 1.5, 7., 5.];
	let mut group_of_plottings = vec![];
	group_of_plottings.push(external_gnuplot::Iteration::new(data_3.iter())
		.set_title("Third"));
	comparison_plotting.add(&group_of_plottings);

	// Plot everything

	comparison_plotting.plot(&2).unwrap();


}
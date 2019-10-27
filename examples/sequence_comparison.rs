use external_gnuplot as ext;
use external_gnuplot::prelude::*;

fn main() {
    // Comparing many iterations

    // Computing the data

    let data_1 = vec![0., 1., 2., 3., 4., 5.];
    let data_2 = vec![0., 1.4, 10., 4.];

    // Arrange everything in a vector

    let iter_1 = ext::Sequence::new(data_1);
    let iter_2 = ext::Sequence::new(data_2);

    // Create comparison and plot

    ext::sequence::Comparison::new([iter_1, iter_2].to_vec())
        .set_title("All together")
        .plot(&1)
        .unwrap();

    // Comparing to iterations in an increasing manner

    // First Sequence
    let data_1 = vec![0., 1., 2., 3., 4., 5.];
    let plotting_1 = external_gnuplot::Sequence::new(&data_1).set_title("First");
    // Add another data
    let data_2 = vec![0., 1.4, 10., 4.];
    let group_of_plottings = vec![external_gnuplot::Sequence::new(&data_2).set_title("Second")];
    let mut comparison_plotting = plotting_1
        .compare_with(group_of_plottings)
        .set_title("More comparisons");
    // Keep adding more
    let data_3 = vec![0.1, 1.5, 7., 5.];
    let group_of_plottings = vec![external_gnuplot::Sequence::new(&data_3).set_title("Third")];
    comparison_plotting.add(group_of_plottings);
    // Plot everything
    comparison_plotting.plot(&"my_serie_name").unwrap();
}

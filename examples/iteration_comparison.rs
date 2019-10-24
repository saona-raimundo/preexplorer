use external_gnuplot::prelude::*;
use external_gnuplot as ext;

fn main() {
    // Comparing many iterations

    // Computing the data

    let data_1 = vec![0., 1., 2., 3., 4., 5.];
    let data_2 = vec![0., 1.4, 10., 4.];

    // Arrange everything in a vector

    let iter_1 = ext::Iteration::new(data_1);
    let iter_2 = ext::Iteration::new(data_2);

    // Create comparison and plot

    ext::iteration::Comparison::new([iter_1, iter_2].to_vec())
        .set_title("All together")
        .plot(&1)
        .unwrap();

    // Comparing to iterations in an increasing manner

    // First iteration

    let data_1 = vec![0., 1., 2., 3., 4., 5.];
    let plotting_1 = ext::Iteration::new(data_1)
        .set_title("First");

    // Add another data

    let data_2 = vec![0., 1.4, 10., 4.];
    let mut comparison_plotting = plotting_1
        .compare_with(vec![ext::Iteration::new(data_2).set_title("Second")])
        .set_title("More comparisons");

    // Keep adding more

    let data_3 = vec![0.1, 1.5, 7., 5.];
    comparison_plotting.add(vec![ext::Iteration::new(data_3).set_title("Third")]);

    // Plot everything

    comparison_plotting.plot(&2).unwrap();

}

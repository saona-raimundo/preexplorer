use preexplorer::prelude::*;

fn main() {
    comparing_interations();

    increasing_comparisons();
}

fn comparing_interations() {
    // Computing the data

    let data_1 = vec![0. as f32, 1., 2., 3., 4., 5.];
    let data_2 = vec![0., 1.4, 10., 4.];

    // Define plotables

    let iter_1 = pre::Sequence::new(data_1.into_iter());
    let iter_2 = pre::Sequence::new(data_2.into_iter());

    // Create comparison and plot

    pre::sequence::Comparison::new([iter_1, iter_2].to_vec())
        .set_title("All together")
        .plot(&1)
        .unwrap();
}

fn increasing_comparisons() {
    // First Sequence

    let data_1 = vec![0., 1., 2., 3., 4., 5.];
    let plotting_1 = pre::Sequence::new(data_1.iter()).set_title("First").to_owned();

    // Another sequence

    let data_2 = vec![0., 1.4, 10., 4.];
    let group_of_plottings = vec![pre::Sequence::new(data_2.iter()).set_title("Second").to_owned()];
    let mut comparison_plotting = plotting_1
        .compare_with(group_of_plottings);
    
    // Keep adding more

    let data_3 = vec![0.1, 1.5, 7., 5.];
    let group_of_plottings = vec![pre::Sequence::new(data_3.iter()).set_title("Third").to_owned()];
    comparison_plotting.add(group_of_plottings);

    // Change some settings

    comparison_plotting.set_title("More comparisons");

    // Plot everything

    comparison_plotting.plot(&"my_serie_name").unwrap();
}

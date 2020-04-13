use preexplorer::prelude::*;

fn main() {
    // Computing the data

    let data_1 = vec![0., 1., 2., 3., 4., 5.];
    let data_2 = vec![0., 1.4, 10., 4.];

    // Arrange everything in a vector

    let group_of_plottings = vec![data_1.preexplore(), data_2.preexplore()];

    pre::Sequences::new(group_of_plottings)
        .title("All together")
        .custom("palette", "red")
        .xrange(0, 20)
        .ticsy("0, 0.5, 1")
        .ytics("0, 0.3, 2")
        .pause(3)
        .plot("1")
        .unwrap();
}

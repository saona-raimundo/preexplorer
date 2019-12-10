use preexplorer::prelude::*;

fn main() {
    // Simpliest use

    let data = 0..10;
    data.preexplore().plot(&0).unwrap();

    // Data is an iterator already

    let data = 0..10;
    let mut plotting = data.preexplore();
    plotting.title("My Title").logx(-1.).plot(1).unwrap();

    // Data is a full array

    let data = vec![0, 1, 2, 3, 4];
    let mut plotting = (&data).preexplore();
    plotting
        .title("My Title 2")
        .logx(-1.)
        .plot("my_plot_serie_name")
        .unwrap();

    println!("{:?}", data);
}

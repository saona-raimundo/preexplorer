use preexplorer::prelude::*;

fn main() {
    // Simpliest use

    let data = 0..10;
    pre::Sequence::new(data).plot(&0).unwrap();

    // Data is an iterator already

    let data = 0..10;
    let mut plotting = pre::Sequence::new(data);
    plotting.set_title("My Title").set_logx(-1.);
    plotting.plot(&1).unwrap();

    // Data is a full array

    let data = vec![0, 1, 2, 3, 4];
    let mut plotting = pre::Sequence::new(data.iter());
    plotting.set_title("My Title 2").set_logx(-1.);
    plotting.plot(&"my_plot_serie_name").unwrap();

    println!("{:?}", data);
}

use external_gnuplot::prelude::*;

fn main() {
    let values = (0..200).chain(0..50);

    ext::Distribution::new(values)
        .set_title("My Title")
        .set_logx(2)
        .plot(&1)
        .unwrap();
}

use external_gnuplot::prelude::*;

fn main() {
    let values_1 = (0..200).chain(0..50).chain(0..50);
    let values_2 = (100..300).chain(100..220).chain(150..250);

    ext::Distribution::new(values_1)
        .set_title("My legend")
        .compare_with(vec![ext::Distribution::new(values_2)])
        .set_title("My title")
        .plot(&1)
        .unwrap();
}

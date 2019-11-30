use preexplorer::prelude::*;

fn main() {
	let values_1: Vec<u32> = (0..200).chain(0..50).collect();
    let values_2: Vec<u32> = (100..300).chain(100..220).chain(150..250).collect();

    pre::Distribution::new(values_1)
        .title("My legend")
        .to_owned()
        .compare_with(vec![pre::Distribution::new(values_2)])
        .title("My title")
        .plot(1)
        .unwrap();
}

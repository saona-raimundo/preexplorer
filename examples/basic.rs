use external_gnuplot::prelude::*;

fn main() {

	// Data is an iterator already

	let data = 0..10;
	let plotting = Iteration::new(data.clone())
		.set_title("My Title")
		.set_logx(-1.);
	plotting.plot(&1).unwrap();

	println!("{:?}", data);

	// Data is a full array

	let data = vec![0, 1, 2, 3, 4];
	let plotting = Iteration::new(data.iter())
		.set_title("My Title 2")
		.set_logx(-1.);
	plotting.plot(&2).unwrap();

	println!("{:?}", data);


}
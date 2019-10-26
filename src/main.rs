use external_gnuplot::prelude::*;
use external_gnuplot as ext;

fn main() {
	let mut data = vec![0, 1, 2, 3, 4];
	let plotting = Sequence::new(&data)
	    .set_title("My Title")
	    .set_logx(-1.); // Default for gnuplot
	plotting.plot(&"my_serie_name").unwrap();

	data[0] = 1;
}

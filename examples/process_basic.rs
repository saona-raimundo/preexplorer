use external_gnuplot::prelude::*;

fn main() {
	let times = vec![1., 10., 100.];
	let values = vec![1, 2, 4];
	ext::Process::new(times, values)
	    .set_title("My Title")
	    .set_logx(-2)
	    .plot(&"my_serie_name").unwrap();

}

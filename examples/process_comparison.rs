use external_gnuplot::prelude::*;

fn main() {
    let times = vec![1., 10., 100.];
    let values = vec![1, 2, 4];
    let plotting = ext::Process::new(times, values)
        .set_title("My Title")
        .set_logx(2);

    let times = vec![1., 10., 100.];
    let values = vec![1, 4, 8];

    let comp = plotting.compare_with(vec![ext::Process::new(times, values)]);
    comp.plot(&"my_serie_name").unwrap();
}

use preexplorer::prelude::*;

fn main() {
    let times = vec![1., 10., 100.];
    let values = vec![1, 2, 4];
    pre::Process::new(times, values)
        .title("My Title")
        .logx(-2)
        .plot("my_serie_name")
        .unwrap();
}

use preexplorer::prelude::*;

fn main() {
    let times = vec![1., 10., 100.];
    let values = vec![1, 2, 4];
    (times, values)
        .preexplore()
        .title("My Title")
        .logx(-2)
        .id("my_serie_name")
        .plot("1")
        .unwrap();
}

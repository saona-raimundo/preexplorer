use preexplorer::prelude::*;

fn main() {
    let times = vec![1., 10., 100.];
    let values = vec![1, 2, 4];
    let plotting = (times, values)
        .preexplore()
        .title("My Title")
        .logx(2)
        .to_owned();

    let times = vec![1., 10., 100.];
    let values = vec![1, 4, 8];

    let mut comp = plotting.compare_with(vec![(times, values).preexplore()]);
    comp.plot("my_serie_name").unwrap();
}

use preexplorer::prelude::*;

fn main() {
    let values: Vec<u32> = (0..200).chain(0..50).collect();

    pre::Distribution::new(values)
        .set_title("My Title")
        .set_logx(2)
        .plot(1)
        .unwrap();
}

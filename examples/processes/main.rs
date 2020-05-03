use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    let times = vec![1., 10., 100.];
    let values = vec![1, 2, 4];
    let plotting = (times, values)
        .preexplore()
        .set_title("My Title")
        .set_logx(2) // Will be forgotten in the comparison plot
        .to_owned();

    let times = vec![1., 10., 100.];
    let values = vec![1, 4, 8];

    plotting
        .compare_with(vec![(times, values).preexplore()])
        .plot("my_serie_name")?;

    Ok(())
}

use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    let times = vec![1., 10., 100.];
    let values = vec![1, 2, 4];
    (times, values)
        .preexplore()
        .set_title("My Title")
        .set_logx(-3) // Negative values imply logx(10), gnuplot sintaxis
        .plot("identifier")?;

    Ok(())
}

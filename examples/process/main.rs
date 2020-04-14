use preexplorer::prelude::*;

fn main() -> failure::Fallible<()> {
    let times = vec![1., 10., 100.];
    let values = vec![1, 2, 4];
    (times, values)
        .preexplore()
        .title("My Title")
        .logx(-3) // Negative values imply logx(10), gnuplot sintaxis
        .plot("identifier")?;

    Ok(())
}

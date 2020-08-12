use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    // From coordinate values
    let xs = vec![1., 10., 100.];
    let ys = vec![1., 20., 50.];
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    pre::Heatmap::new(xs, ys, values)
        .set_title("My Title")
        .set_xlabel("xs")
        .set_ylabel("ys")
        .plot("my_identifier")?;

    // From a matrix
    let matrix = ndarray::arr2(&[[3, 6, 9], [2, 5, 8], [1, 4, 7]]);
    pre::Heatmap::from(matrix)
        .set_title("My Title")
        .set_xlabel("xs")
        .set_ylabel("ys")
        .plot("my_identifier")?;

    Ok(())
}

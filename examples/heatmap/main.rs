use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    // From coordinate values
    let xs = vec![1., 10., 100., 300.];
    let ys = vec![1., 20., 100., 150.];
    let values: Vec<usize> = (1..=16).collect();
    pre::Heatmap::new(xs, ys, values)
        .set_title("My Title")
        .set_xlabel("xs")
        .set_ylabel("ys")
        .plot("my_identifier")?;

    // From a matrix (coordinates are replaced by simple enumeration)
    let matrix = ndarray::arr2(&[
        [4, 8, 12, 16],
        [3, 7, 11, 15],
        [2, 6, 10, 14],
        [1, 5, 9, 13],
    ]);
    pre::Heatmap::from(matrix)
        .set_title("My Title 2")
        .set_xlabel("xs")
        .set_ylabel("ys")
        .plot("my_identifier_2")?;

    Ok(())
}

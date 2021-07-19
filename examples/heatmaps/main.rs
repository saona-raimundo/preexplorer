use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    // From coordinate values
    let xs = vec![1, 10, 100, 300];
    let ys = vec![1, 20, 100, 150];
    let values: Vec<_> = (1..=16).collect();
    let heatmap = pre::Heatmap::new(xs, ys, values)
        .set_title("My first title")
        .set_xlabel("xs")
        .set_ylabel("ys")
        .to_owned();

    // From a matrix
    let matrix = ndarray::arr2(&[
        [4, 8, 12, 16],
        [3, 7, 11, 15],
        [2, 6, 10, 14],
        [1, 5, 9, 13],
    ]);
    (heatmap
        + pre::Heatmap::from(matrix)
            .set_title("My second title")
            .set_xlabel("xs")
            .set_ylabel("ys")
            .to_owned())
    .set_title("Overall title")
    .plot("my_identifier")?;

    Ok(())
}

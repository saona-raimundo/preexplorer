use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    // From coordinate values
    let xs = vec![1, 10, 100];
    let ys = vec![1, 20, 50];
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let heatmap = pre::Heatmap::new(xs, ys, values)
        .set_title("first")
        .to_owned();

    // From a matrix
    let matrix = ndarray::arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    (heatmap + 
        pre::Heatmap::from(matrix)
            .set_title("second")
            .to_owned()
        )
        .set_title("Overall title")
        .plot("my_identifier")?;

    Ok(())
}

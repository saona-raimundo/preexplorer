use ndarray::array;
use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    let data = array![[1, 2, 3, 4, 5], [2, 5, 6, 7, 8], [3, 11, 12, 13, 14],];
    let dim = 5;

    pre::Data::new(data.iter(), dim)
        .set_title("My Title")
        .plot_later("1")?;

    Ok(())
}

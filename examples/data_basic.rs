use ndarray::array;
use preexplorer::prelude::*;

fn main() -> failure::Fallible<()> {
    let data = array![[1, 2], [2, 5], [3, 11],];
    let dim = 2;

    pre::Data::new(data.iter(), dim)
        .title("My Title")
        .plot(1)?;

    Ok(())
}

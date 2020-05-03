use itertools::Itertools;
use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    let mean = (0..10).map(|i| i as f64);
    let error = (0..10).map(|_| rand::random::<f64>());

    let data = mean.interleave(error);
    let dim = 2;

    pre::Data::new(data, dim)
        .set_title("Numerical results")
        .plot_later("my_identifier")?;

    Ok(())
}

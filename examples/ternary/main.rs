use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    // From three dimensional points
    let values = (1..=16).map(|i| i as f64 / 16.0).map(|x| [1.0 - x, x / 2.0, x / 2.0]);
    pre::Ternary::new(values)
        .set_title("My Title")
        .set_xlabel("p1")
        .set_ylabel("p2")
        .set_zlabel("p3")
        .plot("my_identifier")?;

    Ok(())
}

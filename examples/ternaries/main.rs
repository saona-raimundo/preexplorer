use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    // From three dimensional points
    let values = (1..=16).map(|i| i as f64 / 16.0).map(|x| [1.0 - x, x / 2.0, x / 2.0]);
    let ternary = pre::Ternary::new(values)
        .set_title("My first title")
        .to_owned();

    let values = (1..=16).map(|i| i as f64 / 16.0).map(|x| [x / 2.0, 1.0 - x, x / 2.0]);
    let ternary2 = pre::Ternary::new(values)
        .set_title("My second title")
        .to_owned();

    (ternary + ternary2)
        .set_xlabel("p1")
        .set_ylabel("p2")
        .set_zlabel("p3")
        .set_title("Overall title")
        .plot("my_identifier")?;

    Ok(())
}

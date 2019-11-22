use preexplorer::prelude::*;

fn main() -> failure::Fallible<()> {
    let data = 0..7;
    let dim = 2;

    pre::Data::new(data, dim).set_title("My legend").plot(&1)?;

    Ok(())
}

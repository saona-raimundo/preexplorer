use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    let values = (1..10).chain(1..5).chain(1..5);

    pre::Density::new(values)
        .set_title("My empirical density")
        .set_logx(2)
        .plot("my identifier")?;

    Ok(())
}

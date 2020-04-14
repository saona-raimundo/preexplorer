use preexplorer::prelude::*;

fn main() -> failure::Fallible<()> {
    let values = (1..20).chain(1..10).chain(1..10);

    pre::Density::new(values)
        .title("My empirical density")
        .logx(2)
        .plot("my identifier")?;

    Ok(())
}

use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    let data: Vec<Vec<f64>> = (0..10)
        .map(|i| {
            (0..10)
                .map(|j| {
                    // Some computation
                    (i as f64).sin() * j as f64
                })
                .collect()
        })
        .collect();

    pre::SequenceError::new(data)
        .set_title("Numerical results with variable error")
        .set_xlabel("index")
        .set_ylabel("value")
        .plot("my_identifier")?;

    Ok(())
}

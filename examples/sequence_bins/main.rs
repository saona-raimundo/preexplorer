use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    let data_1: Vec<Vec<f64>> = (1..15)
        .map(|i| {
            (0..10)
                .map(|j| {
                    let j = j as f64;
                    let i = i as f64;
                    // Some computation
                    i + j / i
                })
                .collect()
        })
        .collect();
    let data_2: Vec<Vec<f64>> = (1..20)
        .map(|i| {
            (0..10)
                .map(|j| {
                    let j = j as f64;
                    let i = i as f64;
                    // Some computation
                    - i + j / i
                })
                .collect()
        })
        .collect();

    let binwidth = 0.3;

    (pre::SequenceBin::new(data_1, binwidth).set_title("first").to_owned()
        + pre::SequenceBin::new(data_2, binwidth).set_title("second").to_owned()
        )
        .set_xlabel("index")
        .set_ylabel("value")
        .set_title("Overall title")
        .plot("my_identifier")?;

    Ok(())
}

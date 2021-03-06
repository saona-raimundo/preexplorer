use preexplorer::prelude::*;

fn main() -> anyhow::Result<()> {
    let domain = (1..15).map(|i| (i as f64).sqrt());
    let image: Vec<Vec<f64>> = (1..15)
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

    pre::ProcessViolin::new(domain, image)
        .set_title("Numerical results through histograms")
        .set_xlabel("index")
        .set_ylabel("value")
        .plot("my_identifier")?;

    Ok(())
}
